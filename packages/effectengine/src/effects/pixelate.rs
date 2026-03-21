use std::io::Cursor;
#[cfg(not(target_arch = "wasm32"))]
use std::process::exit;
use wasm_bindgen::prelude::*;

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};

#[cfg(not(target_arch = "wasm32"))]
use crate::util::subcommand_help_requested;
use crate::util::{get_paths, read_image};

/// Applies a pixelation filter to an image by combining multiple pixels into bigger ones.
/// Calculates the average color of each "big pixel" to do so.
#[wasm_bindgen(js_name = pixelate)]
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
    let image_width = image.width();
    let image_height = image.height();

    // How big the pixels in the final image should be
    let processed_pixel_size = std::env::args()
        .nth(4)
        .or_else(|| Some(String::from("16")))
        .unwrap()
        .parse()
        .unwrap_or_else(|_| 16);

    let mut new_image = ImageBuffer::new(image_width, image_height);

    for i in (0..image_width).step_by(processed_pixel_size as usize) {
        for j in (0..image_height).step_by(processed_pixel_size as usize) {
            let mut r_sum: u64 = 0;
            let mut g_sum: u64 = 0;
            let mut b_sum: u64 = 0;
            let mut a_sum: u64 = 0;
            let mut count: u64 = 0;

            let x_end: u32 = (i + processed_pixel_size).min(image_width);
            let y_end: u32 = (j + processed_pixel_size).min(image_height);

            for k in i..x_end {
                for l in j..y_end {
                    let pixel = image.get_pixel(k, l);

                    r_sum += pixel.0[0] as u64;
                    g_sum += pixel.0[1] as u64;
                    b_sum += pixel.0[2] as u64;
                    a_sum += pixel.0[3] as u64;
                    count += 1;
                }
            }

            if count > 0 {
                let color = Rgba([
                    (r_sum / count) as u8,
                    (g_sum / count) as u8,
                    (b_sum / count) as u8,
                    (a_sum / count) as u8,
                ]);

                for k in i..x_end {
                    for l in j..y_end {
                        new_image.put_pixel(k, l, color);
                    }
                }
            }
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
Pixelation Effect
Applies a pixelation filter to an image by combining multiple pixels into bigger
ones.

USAGE:
  effectengine-cli pixelate <INPUT_PATH> <OUTPUT_PATH> [PIXELATION_STRENGTH]

ARGUMENTS:
  <INPUT_PATH>             The path to an input image that should be processed.
  <OUTPUT_PATH>            The path where the resulting image should be saved.
                           Needs to include the filename.
  [PIXELATION_STRENGTH]    Optional. How strong the pixelation effect should be.
                           Specifies the size of each big pixel. (Default: 16)
  "#
    );
}
