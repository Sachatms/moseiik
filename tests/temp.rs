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
/// Uses large image to test complete pipeline with SSE2/AVX2
#[test]
#[ignore] // Slow test - run with: cargo test -- --ignored
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_x86() {
    let output = "test_output_x86.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(true, output);
}

/// Integration test for ARM NEON SIMD implementation
/// Uses large image to test complete pipeline with NEON
#[test]
#[ignore] // Slow test - run with: cargo test -- --ignored
#[cfg(target_arch = "aarch64")]
fn test_aarch64() {
    let output = "test_output_aarch64.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(true, output);
}

/// Integration test for generic (non-SIMD) implementation
/// Uses large image to test complete pipeline without SIMD
#[test]
#[ignore] // Slow test - run with: cargo test -- --ignored
fn test_generic() {
    let output = "test_output_generic.png";
    let _cleanup = TestCleanup::new(output);
    run_integration_test(false, output);
}

/// Helper function for integration tests
fn run_integration_test(use_simd: bool, output_path: &str) {
    use moseiik::main::Options;

    let args = Options {
        image: "assets/kit.jpeg".to_string(),
        tiles: "assets/images".to_string(),
        tile_size: 25,
        output: output_path.to_string(),
        verbose: false,
        scaling: 1,
        num_thread: 4,
        simd: use_simd,
        remove_used: false,
    };

    compute_mosaic(args);

    // Verify output exists and dimensions are reasonable
    // Note: These dimensions are specific to assets/kit.jpeg (1920x1080)
    // with 25x25 tiles. The mosaic algorithm may adjust final dimensions
    // to multiples of tile size (1920/25=76.8 -> 76 tiles * 25 = 1900px width).
    let output_img = image::open(output_path).expect("Failed to open output");
    assert!(
        output_img.width() >= 1900 && output_img.width() <= 1920,
        "Output width {} is outside expected range [1900-1920]",
        output_img.width()
    );
    assert!(
        output_img.height() >= 1050 && output_img.height() <= 1080,
        "Output height {} is outside expected range [1050-1080]",
        output_img.height()
    );
}
