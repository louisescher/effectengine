use std::io::Cursor;
#[cfg(not(target_arch = "wasm32"))]
use std::process::exit;

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};

#[cfg(not(target_arch = "wasm32"))]
use crate::util::subcommand_help_requested;
use crate::util::{get_paths, read_image};

/// An implementation of bayer dithering for colored images. Takes in an image to dither
/// and a dithering kernel/bayer matrix that is used for the threshold lookups.
///
/// Given a bayer matrix of a certain size (2, 4, 8, 16, ...), this function will go
/// through an image and check whether the R, G, and B component of the pixel are
/// above or below the threshold. Should the channel be above the threshold, it'll be
/// exaggerated, should the channel be below the threshold, it'll be omitted.
pub fn apply_diffusion_kernel(kernel_size: usize, _kernel: Vec<u8>) -> Vec<u8> {
    let kernel: Vec<Vec<u8>> = _kernel
        .chunks_exact(kernel_size)
        .map(|x| x.to_vec())
        .collect();

    let kernel_height = kernel.len() as u32;
    let kernel_width = kernel[0].len() as u32;

    #[cfg(not(target_arch = "wasm32"))]
    {
        if subcommand_help_requested() {
            print_help(kernel_height);
            exit(0);
        }
    }

    let paths = get_paths();
    let image_data = read_image(paths.input_path);

    let img =
        image::load_from_memory(&image_data.data).expect("Failed to decode image from memory");
    let image = img.to_rgba8();

    let (image_width, image_height) = image.dimensions();

    let mut new_image = ImageBuffer::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel = image.get_pixel(x, y);

            let threshold =
                kernel[(y % kernel_height) as usize][(x % kernel_width) as usize] as f32;

            let r = if (pixel[0] as f32) > threshold {
                255
            } else {
                0
            };
            let g = if (pixel[1] as f32) > threshold {
                255
            } else {
                0
            };
            let b = if (pixel[2] as f32) > threshold {
                255
            } else {
                0
            };

            new_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
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
fn print_help(matrix_size: u32) {
    println!(
        r#"
Bayer Dithering Effect
Approximates an image using a {matrix_size} by {matrix_size} dithering matrix and
only full red, green and blue colors in combination.

USAGE:
  effectengine bayer-{matrix_size} <INPUT_PATH> <OUTPUT_PATH>

ARGUMENTS:
  <INPUT_PATH>     The path to an input image that should be processed.
  <OUTPUT_PATH>    The path where the resulting image should be saved.
                   Needs to include the filename.
  "#
    );
}
