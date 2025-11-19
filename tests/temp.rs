use moseiik::{Options, compute_mosaic};

/// RAII guard to ensure cleanup of test output files
struct TestCleanup {
    path: String,
}

impl TestCleanup {
    fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Drop for TestCleanup {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Integration test for x86/x86_64 SIMD implementation
/// Tests complete pipeline with small dataset (target-small.png + tiles-small)
#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_x86() {
    let output = "test_output_x86.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(true, output);
}

/// Integration test for ARM NEON SIMD implementation
/// Tests complete pipeline with small dataset (target-small.png + tiles-small)
#[test]
#[cfg(target_arch = "aarch64")]
fn test_aarch64() {
    let output = "test_output_aarch64.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(true, output);
}

/// Integration test for generic (non-SIMD) implementation
/// Tests complete pipeline with small dataset (target-small.png + tiles-small)
#[test]
fn test_generic() {
    let output = "test_output_generic.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(false, output);
}

/// Helper function for integration tests
/// Uses small test data (tiles-small, target-small.png) for fast CI testing
fn run_integration_test(use_simd: bool, output_path: &str) {
    let args = Options {
        output: output_path.to_string(),
        num_thread: 4,
        simd: use_simd,
        ..Default::default()
    };

    compute_mosaic(args);

    // Verify output exists and can be loaded
    let output_img = image::open(output_path)
        .expect("Failed to open output")
        .into_rgb8();

    // The target-small.png should be reconstructed perfectly since tiles-small
    // contains the exact tiles from the original image
    let target_img = image::open("assets/target-small.png")
        .expect("Failed to open target")
        .into_rgb8();

    // Pixel-perfect comparison - tiles-small should reconstruct target-small exactly
    assert_eq!(
        output_img, target_img,
        "Output should be pixel-perfect identical to target for tiles-small dataset"
    );
}

/// Integration test for ground truth verification with full dataset
/// Tests complete pipeline with kit.jpeg and downloaded tiles
/// This test is ignored by default as it requires downloading the test images dataset
#[test]
#[ignore]
fn test_ground_truth_kit() {
    // Check if downloaded tiles exist
    let tiles_path = "moseiik_test_images";
    if !std::path::Path::new(tiles_path).exists() {
        eprintln!(
            "⚠️  Skipping ground truth test - tiles not found at '{}'",
            tiles_path
        );
        eprintln!(
            "   Download from: https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip"
        );
        return;
    }

    let output = "test_ground_truth.png";
    let _cleanup = TestCleanup::new(output);

    let args = Options {
        image: "assets/kit.jpeg".to_string(),
        tiles: tiles_path.to_string(),
        tile_size: 25,
        scaling: 1,
        output: output.to_string(),
        num_thread: 4,
        simd: true,
        ..Default::default()
    };

    compute_mosaic(args);

    let output_img = image::open(output)
        .expect("Failed to open generated output")
        .into_rgb8();
    let ground_truth = image::open("assets/ground-truth-kit.png")
        .expect("Failed to open ground truth image")
        .into_rgb8();

    // Verify dimensions first
    assert_eq!(
        output_img.dimensions(),
        ground_truth.dimensions(),
        "Image dimensions don't match! Output: {:?}, Ground truth: {:?}",
        output_img.dimensions(),
        ground_truth.dimensions()
    );

    // Pixel-perfect comparison against known ground truth
    assert_eq!(
        output_img, ground_truth,
        "Generated mosaic should be pixel-perfect identical to ground truth. \
         Dimensions match but pixel values differ."
    );
}
