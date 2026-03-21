use std::{io::Cursor, process::exit};
use wasm_bindgen::prelude::*;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Rgba};

use crate::util::{get_paths, hex_to_rgb, is_hex_color, pixel_to_grayscale_value, read_image};

#[cfg(not(target_arch = "wasm32"))]
use crate::util::subcommand_help_requested;

/// An implementation of the Floyd-Steinberg dithering algorithm. When a pixel's error is calculated, the
/// error is diffused down to other pixels with the following pattern (X is the current pixel, the numbers
/// are fractions of 16):
///
/// ```
/// |- - -|- - -|- - -|
/// |     |  x  |  7  |
/// |- - -|- - -|- - -|
/// |  3  |  5  |  1  |
/// |- - -|- - -|- - -|
/// ```
///
/// The algorithm does this by storing the diffused error in a one-dimensional array with the size equal
/// to the width of the image. Due to the divisor being 16, which is a multiple of two, bit-shifting can
/// be used for better performance.
#[wasm_bindgen(js_name = floydSteinberg)]
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

    let image = DynamicImage::ImageRgba8(
        image::load_from_memory(&image_data.data)
            .expect("Failed to decode image from memory")
            .to_rgba8(),
    );

    let pixels = image.pixels();
    let image_width = image.width() as usize;

    let dark_color_hex = std::env::args().nth(4).unwrap_or(String::from("#000000"));
    let light_color_hex = std::env::args().nth(5).unwrap_or(String::from("#FFFFFF"));

    if !is_hex_color(dark_color_hex.clone()) || !is_hex_color(light_color_hex.clone()) {
        eprintln!("Colors must be provided in 6 part hexadecimal format (#000000).");
        exit(64);
    }

    let dark_color = hex_to_rgb(dark_color_hex);
    let light_color = hex_to_rgb(light_color_hex);

    let mut diffusion_array: Vec<i32> = vec![0; image_width + 1];
    let mut diff_array_for_row: Vec<i32> = vec![0; image_width + 1];
    let mut next_diff_err: i32 = 0;
    let mut current_row: usize = 0;

    let mut new_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(image_width as u32, image.height());

    for (i, pixel) in pixels.enumerate() {
        // Check if we're in a different row by now
        let row_check = i / image_width;
        if row_check > current_row {
            current_row += 1;
            next_diff_err = 0;

            // First, swap the arrays so we can work with the errors from previous rows
            std::mem::swap(&mut diff_array_for_row, &mut diffusion_array);

            // Next, clear the original diffusion array so it's good to work with again
            for v in &mut diffusion_array {
                *v = 0;
            }
        }

        let proper_index = i - (image_width * current_row);

        // Now we start computing the actual pixel errors
        let pixel_color = pixel_to_grayscale_value(pixel);

        // Factor in the errors from previous pixels
        let adjusted_pixel_color =
            pixel_color + next_diff_err + diff_array_for_row[proper_index].clamp(0, 255);

        let pixel_error = if adjusted_pixel_color < 128 {
            new_image.put_pixel(pixel.0, pixel.1, dark_color);

            adjusted_pixel_color
        } else {
            new_image.put_pixel(pixel.0, pixel.1, light_color);

            adjusted_pixel_color - 255
        };

        // The error for the next pixel to be processed.
        next_diff_err = pixel_error * 7 >> 4;

        // The errors for the next row of pixels, left to right.
        // In cases where we're on the first pixel of a row, we can't push to the bottom left pixel.
        if proper_index > 0 {
            diffusion_array[proper_index - 1] += pixel_error * 3 >> 4;
        }

        // This is the value for the pixel that's right below ours!
        diffusion_array[proper_index] += pixel_error * 5 >> 4;

        // Lastly, the pixel to the bottom right.
        // In cases where we're at the last pixel of a row, this pixel doesn't exist.
        if proper_index < image_width - 1 {
            diffusion_array[proper_index + 1] += pixel_error >> 4;
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
Floyd Steinberg Dithering Effect
Approximates an image using only black and white pixels.

USAGE:
  effectengine-cli floyd-steinberg <INPUT_PATH> <OUTPUT_PATH> [DARK_COLOR] [LIGHT_COLOR]

ARGUMENTS:
  <INPUT_PATH>     The path to an input image that should be processed.
  <OUTPUT_PATH>    The path where the resulting image should be saved.
                   Needs to include the filename.
  [DARK_COLOR]     Optional. The color that should be used for the dark
                   pixels. Specified as a full-length hexadecimal color.
                   (Default: #000000)
  [LIGHT_COLOR]    Optional. The color that should be used for the light
                   pixels. Specified as a full-length hexadecimal color.
                   (Default: #FFFFFF)
  "#
    );
}
