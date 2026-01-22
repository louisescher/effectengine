use std::process::exit;
use rand::Rng;
use wasm_bindgen::prelude::*;

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::util::subcommand_help_requested;

/// Applies white noise to the given image.
#[wasm_bindgen(js_name = whiteNoise)]
pub fn effect(data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
	if subcommand_help_requested() {
		print_help();
		exit(0);
	}

	let opacity: i32 = std::env::args().nth(4).or_else(|| {
		Some(String::from("32"))
	}).unwrap().parse().unwrap_or_else(|_| 32);

	let opacity_factor = opacity as f32 / 255.0;

	let image = RgbaImage::from_raw(width, height, data.to_vec()).expect("Container should be large enough for the pixels");
	let mut new_image = ImageBuffer::new(image.width(), image.height());

	for x in 0..image.width() {
		for y in 0..image.height() {
			let pixel = image.get_pixel(x, y);
			let noise_value = rand::rng().random_range(0..255);

			let r = (pixel.0[0] as f32 + (1.0 - opacity_factor) + noise_value as f32 * opacity_factor) as u8;
			let g = (pixel.0[1] as f32 + (1.0 - opacity_factor) + noise_value as f32 * opacity_factor) as u8;
			let b = (pixel.0[2] as f32 + (1.0 - opacity_factor) + noise_value as f32 * opacity_factor) as u8;
			let a = (pixel.0[3] as f32 + (1.0 - opacity_factor) + noise_value as f32 * opacity_factor) as u8;

			new_image.put_pixel(x, y, Rgba([r, g, b, a]));
		}
	}

	return new_image.as_raw().clone();
}


/// Prints the help text for this effect.
fn print_help() {
	println!(r#"
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
  "#);
}
