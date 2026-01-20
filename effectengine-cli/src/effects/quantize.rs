use std::{path::PathBuf, process::exit};

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba};

use crate::util::{hex_to_rgb, is_hex_color, pixel_to_grayscale_value};

/// Tries to quantize a color to one between a given few.
pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let image_width = image.width();
	let image_height = image.height();

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

	colors.sort_by(|a, b| {
		let grayscale_a = pixel_to_grayscale_value((0, 0, *a));
		let grayscale_b = pixel_to_grayscale_value((0, 0, *b));

		grayscale_a.cmp(&grayscale_b)
	});

	let mut new_image = ImageBuffer::new(image_width, image_height);

	for pixel in image.pixels() {
		let mut distances: Vec<i32> = Vec::new();

		for color in &colors {
			let distance = distance_between_colors(pixel.2, *color);

			distances.push(distance);
		}

		let lowest = distances.iter().min().unwrap().clone();
		let mut index = 0;

		for value in distances {
			if value == lowest {
				break;
			}

			index += 1;
		}

		new_image.put_pixel(pixel.0, pixel.1, colors[index]);
	}

	return new_image;
}

fn distance_between_colors(color_1: Rgba<u8>, color_2: Rgba<u8>) -> i32 {
	let r1 = color_1.0[0] as f64;
	let g1 = color_1.0[1] as f64;
	let b1 = color_1.0[2] as f64;

	let r2 = color_2.0[0] as f64;
	let g2 = color_2.0[1] as f64;
	let b2 = color_2.0[2] as f64;

	let dr = r1 - r2;
	let dg = g1 - g2;
	let db = b1 - b2;

	// Weighted Euclidean distance formula
	// Weights: Red = 0.299, Green = 0.587, Blue = 0.114
	(0.299 * dr * dr + 0.587 * dg * dg + 0.114 * db * db).sqrt().round() as i32
}
