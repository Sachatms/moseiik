use moseiik::main::compute_mosaic;
use std::fs;
use std::path::Path;

/// Integration test for x86/x86_64 SIMD implementation
/// Uses large image to test complete pipeline with SSE2/AVX2
#[test]
#[ignore] // Slow test - run with: cargo test -- --ignored
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_x86() {
    let output = "test_output_x86.png";
    run_integration_test(true, output);
    assert!(Path::new(output).exists(), "Output file should exist");
    cleanup(output);
}

/// Integration test for ARM NEON SIMD implementation
/// Uses large image to test complete pipeline with NEON
#[test]
#[ignore] // Slow test - run with: cargo test -- --ignored
#[cfg(target_arch = "aarch64")]
fn test_aarch64() {
    let output = "test_output_aarch64.png";
    run_integration_test(true, output);
    assert!(Path::new(output).exists(), "Output file should exist");
    cleanup(output);
}

/// Integration test for generic (non-SIMD) implementation
/// Uses large image to test complete pipeline without SIMD
#[test]
#[ignore] // Slow test - run with: cargo test -- --ignored
fn test_generic() {
    let output = "test_output_generic.png";
    run_integration_test(false, output);
    assert!(Path::new(output).exists(), "Output file should exist");
    cleanup(output);
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
    let output_img = image::open(output_path).expect("Failed to open output");
    // Mosaic tiles may adjust final dimensions slightly
    assert!(output_img.width() >= 1900 && output_img.width() <= 1920);
    assert!(output_img.height() >= 1050 && output_img.height() <= 1080);
}

/// Cleanup helper
fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
}
