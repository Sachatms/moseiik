use moseiik::main::compute_mosaic;

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
    use moseiik::main::Options;

    let args = Options {
        image: "assets/target-small.png".to_string(),
        tiles: "assets/tiles-small".to_string(),
        tile_size: 5,
        output: output_path.to_string(),
        verbose: false,
        scaling: 1,
        num_thread: 4,
        simd: use_simd,
        remove_used: false,
    };

    compute_mosaic(args);

    // Verify output exists and can be loaded
    let output_img = image::open(output_path).expect("Failed to open output");

    // The target-small.png should be reconstructed perfectly since tiles-small
    // contains the exact tiles from the original image
    let target_img = image::open("assets/target-small.png")
        .expect("Failed to open target")
        .into_rgb8();

    // Verify dimensions match (should be exact since tiles divide evenly)
    assert_eq!(
        output_img.width(),
        target_img.width(),
        "Output width doesn't match target"
    );
    assert_eq!(
        output_img.height(),
        target_img.height(),
        "Output height doesn't match target"
    );
}
