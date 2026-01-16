use std::{env::args, process::exit};

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::util::{hex_to_rgb, is_hex_color};

/// Tries to quantize a color to one between a given few.
pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let image_width = image.width();
	let image_height = image.height();

	let mut colors: Vec<Rgba<u8>> = Vec::new();

	for (i, arg) in std::env::args().enumerate() {
		if i < 4 { continue; }

		if !is_hex_color(arg.clone()) {
			eprintln!("All arguments after the file paths must be colors in full hexadecimal format (#000000)!");
			exit(64);
		}

		colors.push(hex_to_rgb(arg));
	}

	dbg!(colors);

	// TODO: Order colors by lightness, then check for each pixels r g and b values and which
	// color they're closest to, then write that color instead

	let mut new_image = ImageBuffer::new(image_width, image_height);

	return new_image;
}
