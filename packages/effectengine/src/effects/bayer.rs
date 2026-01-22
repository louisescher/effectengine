use std::process::exit;

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::util::subcommand_help_requested;

/// An implementation of bayer dithering for colored images. Takes in an image to dither
/// and a dithering kernel/bayer matrix that is used for the threshold lookups.
///
/// Given a bayer matrix of a certain size (2, 4, 8, 16, ...), this function will go
/// through an image and check whether the R, G, and B component of the pixel are
/// above or below the threshold. Should the channel be above the threshold, it'll be
/// exaggerated, should the channel be below the threshold, it'll be omitted.
pub fn apply_diffusion_kernel(
	data: Vec<u8>,
	kernel_size: usize,
	_kernel: Vec<u8>,
	width: u32,
	height: u32
) -> Vec<u8> {
	let image = RgbaImage::from_raw(width, height, data.to_vec()).expect("Container should be large enough for the pixels");
	let kernel: Vec<Vec<u8>> = _kernel.chunks_exact(kernel_size).map(|x| x.to_vec()).collect();

	let kernel_height = kernel.len() as u32;
	let kernel_width = kernel[0].len() as u32;

	if subcommand_help_requested() {
		print_help(kernel_height);
		exit(0);
	}

	let (image_width, image_height) = image.dimensions();

	let mut new_image = ImageBuffer::new(image_width, image_height);

	for y in 0..image_height {
		for x in 0..image_width {
			let pixel = image.get_pixel(x, y);

			let threshold = kernel[(y % kernel_height) as usize][(x % kernel_width) as usize] as f32;

			let r = if (pixel[0] as f32) > threshold { 255 } else { 0 };
			let g = if (pixel[1] as f32) > threshold { 255 } else { 0 };
			let b = if (pixel[2] as f32) > threshold { 255 } else { 0 };

			new_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
		}
	}

	return new_image.as_raw().clone();
}

/// Prints the help text for this effect.
fn print_help(matrix_size: u32) {
	println!(r#"
Bayer Dithering Effect
Approximates an image using a {matrix_size} by {matrix_size} dithering matrix and
only full red, green and blue colors in combination.

USAGE:
  effectengine-cli bayer-{matrix_size} <INPUT_PATH> <OUTPUT_PATH>

ARGUMENTS:
  <INPUT_PATH>     The path to an input image that should be processed.
  <OUTPUT_PATH>    The path where the resulting image should be saved.
                   Needs to include the filename.
  "#);
}
