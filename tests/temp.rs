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

/// Check if the test dataset is available
fn dataset_available() -> bool {
    std::path::Path::new("moseiik_test_images/images").exists()
        && std::path::Path::new("moseiik_test_images/kit.jpeg").exists()
        && std::path::Path::new("moseiik_test_images/output.jpeg").exists()
}

/// Integration test for x86/x86_64 SIMD implementation
/// Uses the full moseiik_test_images dataset
#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_x86() {
    if !dataset_available() {
        eprintln!("⚠️  Skipping test - moseiik_test_images dataset not found");
        eprintln!(
            "   Download from: https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip"
        );
        return;
    }

    let output = "test_output_x86.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(true, output);
}

/// Integration test for ARM NEON SIMD implementation
/// Uses the full moseiik_test_images dataset
#[test]
#[cfg(target_arch = "aarch64")]
fn test_aarch64() {
    if !dataset_available() {
        eprintln!("⚠️  Skipping test - moseiik_test_images dataset not found");
        eprintln!(
            "   Download from: https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip"
        );
        return;
    }

    let output = "test_output_aarch64.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(true, output);
}

/// Integration test for generic (non-SIMD) implementation
/// Uses the full moseiik_test_images dataset
#[test]
fn test_generic() {
    if !dataset_available() {
        eprintln!("⚠️  Skipping test - moseiik_test_images dataset not found");
        eprintln!(
            "   Download from: https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip"
        );
        return;
    }

    let output = "test_output_generic.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(false, output);
}

/// Helper function for integration tests
/// Uses the moseiik_test_images dataset (kit.jpeg + images/)
fn run_integration_test(use_simd: bool, output_path: &str) {
    let args = Options {
        image: "moseiik_test_images/kit.jpeg".to_string(),
        tiles: "moseiik_test_images/images".to_string(),
        output: output_path.to_string(),
        scaling: 1,
        tile_size: 5,
        remove_used: false,
        verbose: false,
        simd: use_simd,
        num_thread: 4,
    };

    compute_mosaic(args);

    // Verify output exists and can be loaded
    let output_img = image::open(output_path)
        .expect("Failed to open output")
        .into_rgb8();

    // Compare against the reference output.jpeg from the dataset
    let reference_img = image::open("moseiik_test_images/output.jpeg")
        .expect("Failed to open reference output")
        .into_rgb8();

    // Pixel-perfect comparison
    assert_eq!(
        output_img, reference_img,
        "Output should be pixel-perfect identical to reference output"
    );
}
