use moseiik::main::{Options, compute_mosaic};
use image::{RgbImage, ImageReader};

fn load_image(path: &str) -> RgbImage {
    ImageReader::open(path)
        .expect(&format!("Cannot open image: {}", path))
        .decode()
        .expect("Failed to decode image")
        .into_rgb8()
}

fn default_options(output: &str) -> Options {
    Options {
        image: "assets/kit.jpeg".into(),          
        output: output.into(),                    
        tiles: "assets/moseiik_test_images/images".into(),       
        scaling: 1,
        tile_size: 25,                             
        remove_used: false,
        verbose: false,
        simd: false,
        num_thread: 1,
    }
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_x86() {
    let output = "tests/output_x86.png";
    let options = default_options(output);

    compute_mosaic(options);

    let generated = load_image(output);
    let ground = load_image("assets/ground-truth-kit.png");

    assert_eq!(generated, ground);

    std::fs::remove_file(output).unwrap();
}

#[test]
#[cfg(target_arch = "aarch64")]
fn test_aarch64() {
    let output = "tests/output_aarch64.png";
    let options = default_options(output);

    compute_mosaic(options);

    let generated = load_image(output);
    let ground = load_image("assets/ground-truth-kit.png");

    assert_eq!(generated, ground);

    std::fs::remove_file(output).unwrap();
}

#[test]
fn test_generic() {
    let output = "tests/output_generic.png";
    let options = default_options(output);

    compute_mosaic(options);

    let generated = load_image(output);
    let ground = load_image("assets/ground-truth-kit.png");

    assert_eq!(generated, ground);

    std::fs::remove_file(output).unwrap();
}
