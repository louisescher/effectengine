#[cfg(not(target_arch = "wasm32"))]
use std::process::exit;
use std::{f32::consts::E, io::Cursor};
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

/// Generates a gaussian kernel for a given sigma value.
fn generate_gaussian_kernel(radius: i32, sigma: f32) -> Vec<f32> {
    let size: usize = (2 * radius + 1) as usize;
    let mut kernel: Vec<f32> = vec![0.0; size];

    let sigma_sq = sigma * sigma;

    let mut sum: f32 = 0.0;

    for i in 0..size {
        let x: f32 = i as f32 - radius as f32;
        kernel[i] = E.powf(-(x * x) / (2.0 * sigma_sq));
        sum += kernel[i];
    }

    for i in 0..size {
        kernel[i] /= sum;
    }

    kernel
}

/// Applies a bloom effect to the given image by gaussian-blurring it and
/// additively blending the result back onto the original.
#[wasm_bindgen(js_name = bloom)]
pub fn effect(data: Vec<u8>, image_format: u8) -> Vec<u8> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if subcommand_help_requested() {
            print_help();
            exit(0);
        }
    }

    let img = image::load_from_memory(&data).expect("Failed to decode image from memory");
    let image = img.to_rgba8();
    let mut tmp_image = ImageBuffer::new(image.width(), image.height());
    let mut new_image = ImageBuffer::new(image.width(), image.height());

    let sigma: f32 = std::env::args()
        .nth(4)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2.0_f32)
        .max(0.1);
    let radius: i32 = 3 * sigma as i32;
    let kernel = generate_gaussian_kernel(radius, sigma);

    let width = image.width() as i32;
    let height = image.height() as i32;

    // First pass, horizontal
    for x in 0..width {
        for y in 0..height {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut a = 0.0;

            for k in -radius..=radius {
                let sx = (x as i32 + k).max(0).min(width - 1);
                let w = kernel[(k + radius) as usize];
                let pixel = image.get_pixel(sx as u32, y as u32);

                r += pixel.0[0] as f32 * w;
                g += pixel.0[1] as f32 * w;
                b += pixel.0[2] as f32 * w;
                a += pixel.0[3] as f32 * w;
            }

            tmp_image.put_pixel(
                x as u32,
                y as u32,
                Rgba([r as u8, g as u8, b as u8, a as u8]),
            );
        }
    }

    // Second pass, vertical
    for x in 0..width {
        for y in 0..height {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut a = 0.0;

            for k in -radius..=radius {
                let sy = (y as i32 + k).max(0).min(height - 1);
                let w = kernel[(k + radius) as usize];
                let pixel = tmp_image.get_pixel(x as u32, sy as u32);

                r += pixel.0[0] as f32 * w;
                g += pixel.0[1] as f32 * w;
                b += pixel.0[2] as f32 * w;
                a += pixel.0[3] as f32 * w;
            }

            let orig = image.get_pixel(x as u32, y as u32);
            let blended_r = (orig.0[0] as f32 + r).min(255.0) as u8;
            let blended_g = (orig.0[1] as f32 + g).min(255.0) as u8;
            let blended_b = (orig.0[2] as f32 + b).min(255.0) as u8;
            let blended_a = (orig.0[3] as f32 + a).min(255.0) as u8;

            new_image.put_pixel(
                x as u32,
                y as u32,
                Rgba([blended_r, blended_g, blended_b, blended_a]),
            );
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
Bloom Effect
Produces a gaussian blur bloom effect by blurring the image and additively
blending the result back onto the original.

USAGE:
  effectengine-cli bloom <INPUT_PATH> <OUTPUT_PATH> [SIGMA]

ARGUMENTS:
  <INPUT_PATH>     The path to an input image that should be processed.
  <OUTPUT_PATH>    The path where the resulting image should be saved.
                   Needs to include the filename.
  [SIGMA]          The sigma (spread) of the gaussian blur. Higher values
                   produce a wider, softer bloom. Defaults to 2.0.
  "#
    );
}
