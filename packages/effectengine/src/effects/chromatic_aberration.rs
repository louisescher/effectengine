use std::io::Cursor;
#[cfg(not(target_arch = "wasm32"))]
use std::process::exit;
use wasm_bindgen::prelude::*;

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};

#[cfg(not(target_arch = "wasm32"))]
use crate::util::subcommand_help_requested;
use crate::util::{get_paths, read_image};

#[wasm_bindgen(start)]
pub fn main_js() {
    // This ensures that any panic in Rust provides a useful error message in the browser console
    console_error_panic_hook::set_once();
}

fn clamp_coord(v: i32, max: u32) -> u32 {
    v.max(0).min(max as i32 - 1) as u32
}

/// Applies a chromatic aberration effect to the given image.
/// Each colour channel (R, G, B) is sampled from a slightly different position,
/// offset radially from the centre of the image, simulating lens colour fringing.
#[wasm_bindgen(js_name = chromaticAberration)]
pub fn effect() -> Vec<u8> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if subcommand_help_requested() {
            print_help();
            exit(0);
        }
    }

    let paths = get_paths();
    let image_data = read_image(paths.input_path);

    let img =
        image::load_from_memory(&image_data.data).expect("Failed to decode image from memory");
    let image = img.to_rgba8();
    let mut new_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(image.width(), image.height());

    let width = image.width();
    let height = image.height();

    // How many pixels the outermost edge is displaced by.
    let strength: f32 = std::env::args()
        .nth(4)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10.0_f32)
        .max(0.0);

    for x in 0..width {
        for y in 0..height {
            // Normalise to -1.0..1.0 relative to the image centre.
            let nx = (x as f32 - width as f32 * 0.5) / (width as f32 * 0.5);
            let ny = (y as f32 - height as f32 * 0.5) / (height as f32 * 0.5);

            // Radial offset vector, scaled by strength.
            let offset_x = (nx * strength).round() as i32;
            let offset_y = (ny * strength).round() as i32;

            // Red channel is shifted outward.
            let rx = clamp_coord(x as i32 + offset_x, width);
            let ry = clamp_coord(y as i32 + offset_y, height);

            // Green channel stays at the original position.
            let gx = x;
            let gy = y;

            // Blue channel is shifted inward (opposite direction).
            let bx = clamp_coord(x as i32 - offset_x, width);
            let by = clamp_coord(y as i32 - offset_y, height);

            let r = image.get_pixel(rx, ry).0[0];
            let g = image.get_pixel(gx, gy).0[1];
            let b = image.get_pixel(bx, by).0[2];
            let a = image.get_pixel(x, y).0[3];

            new_image.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    let mut cursor = Cursor::new(Vec::new());

    if image_data.format == ImageFormat::Jpeg {
        let rgb_image = DynamicImage::ImageRgba8(new_image).into_rgb8();
        rgb_image
            .write_to(&mut cursor, image_data.format)
            .expect("Failed to encode JPEG");
    } else {
        new_image
            .write_to(&mut cursor, image_data.format)
            .expect("Failed to encode image");
    }

    return cursor.into_inner();
}

/// Prints the help text for this effect.
#[cfg(not(target_arch = "wasm32"))]
fn print_help() {
    println!(
        r#"
Chromatic Aberration Effect
Simulates lens colour fringing by displacing the R, G, and B channels
radially outward/inward from the centre of the image.

USAGE:
  effectengine chromatic-aberration <INPUT_PATH> <OUTPUT_PATH> [STRENGTH]

ARGUMENTS:
  <INPUT_PATH>     The path to an input image that should be processed.
  <OUTPUT_PATH>    The path where the resulting image should be saved.
                   Needs to include the filename.
  [STRENGTH]       The maximum pixel displacement at the image edges. Higher
                   values produce more pronounced fringing. Defaults to 10.0.
  "#
    );
}
