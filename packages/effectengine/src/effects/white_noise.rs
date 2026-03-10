use rand::Rng;
use std::io::Cursor;
#[cfg(not(target_arch = "wasm32"))]
use std::process::exit;
use wasm_bindgen::prelude::*;

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};

use crate::util::number_to_image_format;
#[cfg(not(target_arch = "wasm32"))]
use crate::util::subcommand_help_requested;

#[wasm_bindgen(start)]
pub fn main_js() {
    // This ensures that any panic in Rust provides a useful error message in the browser console
    console_error_panic_hook::set_once();
}

/// Applies white noise to the given image.
#[wasm_bindgen(js_name = whiteNoise)]
pub fn effect(data: Vec<u8>, image_format: u8) -> Vec<u8> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if subcommand_help_requested() {
            print_help();
            exit(0);
        }
    }

    let opacity: i32 = std::env::args()
        .nth(4)
        .or_else(|| Some(String::from("32")))
        .unwrap()
        .parse()
        .unwrap_or_else(|_| 32);

    let opacity_factor = opacity as f32 / 255.0;

    let img = image::load_from_memory(&data).expect("Failed to decode image from memory");
    let image = img.to_rgba8();
    let mut new_image = ImageBuffer::new(image.width(), image.height());

    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            let noise_value = rand::rng().random_range(0..255);

            let r = (pixel.0[0] as f32
                + (1.0 - opacity_factor)
                + noise_value as f32 * opacity_factor) as u8;
            let g = (pixel.0[1] as f32
                + (1.0 - opacity_factor)
                + noise_value as f32 * opacity_factor) as u8;
            let b = (pixel.0[2] as f32
                + (1.0 - opacity_factor)
                + noise_value as f32 * opacity_factor) as u8;
            let a = (pixel.0[3] as f32
                + (1.0 - opacity_factor)
                + noise_value as f32 * opacity_factor) as u8;

            new_image.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    let format = number_to_image_format(image_format);
    let mut cursor = Cursor::new(Vec::new());

    if format == ImageFormat::Jpeg {
        let rgb_image = DynamicImage::ImageRgba8(new_image).into_rgb8();
        rgb_image
            .write_to(&mut cursor, format)
            .expect("Failed to encode JPEG");
    } else {
        new_image
            .write_to(&mut cursor, format)
            .expect("Failed to encode image");
    }

    return cursor.into_inner();
}

/// Prints the help text for this effect.
#[cfg(not(target_arch = "wasm32"))]
fn print_help() {
    println!(
        r#"
White Noise Effect
Overlays an image with white noise at a given opacity.

USAGE:
  effectengine-cli white-noise <INPUT_PATH> <OUTPUT_PATH> [OPACITY]

ARGUMENTS:
  <INPUT_PATH>     The path to an input image that should be processed.
  <OUTPUT_PATH>    The path where the resulting image should be saved.
                   Needs to include the filename.
  <OPACITY>        The opacity of the overlaid noise. A number between 0 and
                   255. (Default: 32)
  "#
    );
}
