use std::{collections::HashMap, io::Cursor, path::PathBuf, process::exit};
use wasm_bindgen::prelude::*;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, ImageReader, Rgba};

use crate::util::{hex_to_rgb, is_hex_color, number_to_image_format, pixel_to_grayscale_value};

#[cfg(not(target_arch = "wasm32"))]
use crate::util::subcommand_help_requested;

/// "Quantizes" an image by adjusting the colors to fit a given palette.
/// Each pixel's color is checked for the lowest perceived distance to
/// the color palette, then that new color is written to the new image
/// instead.
#[wasm_bindgen(js_name = quantize)]
pub fn effect(data: Vec<u8>, image_format: u8) -> Vec<u8> {
	#[cfg(not(target_arch = "wasm32"))]
	{
		if subcommand_help_requested() {
			print_help();
			exit(0);
		}
	}

	let image = DynamicImage::ImageRgba8(
		image::load_from_memory(&data).expect("Failed to decode image from memory").to_rgba8()
	);
	let image_width = image.width();
	let image_height = image.height();

	let colors = collect_palette_colors();
	let mut cache: HashMap<[u8; 4], Rgba<u8>> = HashMap::new();

	let mut new_image = ImageBuffer::new(image_width, image_height);

	for (x, y, pixel) in image.pixels() {
		let quantized_color = *cache.entry(pixel.0).or_insert_with(|| {
			find_closest_color(pixel, &colors)
		});

		new_image.put_pixel(x, y, quantized_color);
	}

	let format = number_to_image_format(image_format);
	let mut cursor = Cursor::new(Vec::new());

	if format == ImageFormat::Jpeg {
		let rgb_image = DynamicImage::ImageRgba8(new_image).into_rgb8();
		rgb_image.write_to(&mut cursor, format).expect("Failed to encode JPEG");
	} else {
		new_image.write_to(&mut cursor, format).expect("Failed to encode image");
	}

	return cursor.into_inner();
}

/// Finds the closest color for a given pixel from a given palette.
fn find_closest_color(pixel: Rgba<u8>, palette: &Vec<Rgba<u8>>) -> Rgba<u8> {
	let r1 = pixel.0[0] as f32;
	let g1 = pixel.0[1] as f32;
	let b1 = pixel.0[2] as f32;

	let mut min_dist = f32::MAX;
	let mut closest_color = palette[0];

	for color in palette {
		let dr = r1 - color[0] as f32;
		let dg = g1 - color[1] as f32;
		let db = b1 - color[2] as f32;

		let dist_sq = 0.299 * dr * dr + 0.587 * dg * dg + 0.114 * db * db;

		if dist_sq < min_dist {
			min_dist = dist_sq;
			closest_color = *color;
		}
	}

	closest_color
}

/// Collects the palette colors, either from an input image or from CLI input args.
fn collect_palette_colors() -> Vec<Rgba<u8>> {
	let mut colors: Vec<Rgba<u8>> = Vec::new();

	if std::env::args().len() < 5 || std::env::args().len() == 5 && is_hex_color(std::env::args().nth(4).unwrap()) {
		eprintln!("Need at least two colors or a path to a palette image.");
		exit(64);
	}

	if std::env::args().len() == 5 {
		let palette_path = PathBuf::from(std::env::args().nth(4).unwrap());

		if !palette_path.exists() {
			eprintln!("The given palette was not found.");
			exit(64);
		}

		if !palette_path.is_file() {
			eprintln!("The given palette was not a file.");
			exit(64);
		}

		let palette_reader_res = ImageReader::open(palette_path);

		let palette_reader = match palette_reader_res {
			Ok(_) => palette_reader_res.unwrap(),
			Err(_) => {
				eprintln!("The image at the given input path could not be read.");
				exit(64);
			}
		};

		let palette_image_res = palette_reader.decode();

		let palette_image = match palette_image_res {
			Ok(_) => palette_image_res.unwrap(),
			Err(_) => {
				eprintln!("The palette image at the given path could not be decoded.");
				exit(64);
			}
		};

		for pixel in palette_image.pixels() {
			if colors.iter().find(|&x| *x == pixel.2).is_none() {
				colors.push(pixel.2);
			}
		}
		// Check if the 5th arg is a path to a pallete image, otherwise throw an error saying that more than one color is required
	} else {
		for (i, arg) in std::env::args().enumerate() {
			if i < 4 { continue; }

			if !is_hex_color(arg.clone()) {
				eprintln!("All arguments after the file paths must be colors in full hexadecimal format (#000000)!");
				exit(64);
			}

			colors.push(hex_to_rgb(arg));
		}
	}

	colors.sort_by_key(|c| pixel_to_grayscale_value((0, 0, *c)));

	colors
}

/// Prints the help text for this effect.
#[cfg(not(target_arch = "wasm32"))]
fn print_help() {
	println!(r#"
Quantization Effect
"Quantizes" an image by adjusting the colors to fit a given palette.

USAGE:
  effectengine-cli quantize <INPUT_PATH> <OUTPUT_PATH> [PALETTE_PATH | HEX_CODES...]

ARGUMENTS:
  <INPUT_PATH>      The path to an input image that should be processed.
  <OUTPUT_PATH>     The path where the resulting image should be saved.
                    Needs to include the filename.
  [PALETTE_PATH]    A path to an image, the colors of which should be used as
                    the base palette for the conversion. A good source for
                    palettes is https://lospec.com/palette-list!
  [HEX_CODES...]    A list of hex codes in full format (e.g. #000000 or
                    #FFFFFF). Minimum two.
  "#);
}
