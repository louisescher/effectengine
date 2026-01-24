use std::{io::Cursor, ops::Range, process::exit};
use wasm_bindgen::prelude::*;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Luma, Rgba};

use crate::util::number_to_image_format;
#[cfg(not(target_arch = "wasm32"))]
use crate::util::subcommand_help_requested;

/// Applies an implementation of the Kuwahara filter to the given image.
///
/// The Kuwahara filter is usually used for noise reduction while retaining image
/// quality, but it can be used for artistic purposes because it makes image look
/// "painted". It does this by grabbing small squares from an image, dividing it
/// up into four quadrants, then computing the standard brightness deviation for all
/// pixels inside each quadrant. Whichever quadrant has the lowest deviation then
/// gets used in the next step, where the average color from said quadrant is
/// computed and then applied to the current pixel, which is the center where all
/// four quadrants overlap.
#[wasm_bindgen(js_name = kuwahara)]
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
	let luma8_image = image.to_luma8();

	let mut new_image = ImageBuffer::new(image.width(), image.height());

	let window_size: i32 = std::env::args().nth(4).or_else(|| {
		Some(String::from("5"))
	}).unwrap().parse().unwrap_or_else(|_| 5);

	if window_size < 5 {
		eprintln!("Window needs to be at least 5 pixels wide.");
		exit(64);
	}

	if window_size % 2 == 0 {
		eprintln!("Window needs to have an odd width.");
	}

	let image_width = image.width() as i32;
	let image_height = image.height() as i32;

	for x in 0..image_width {
		for y in 0..image_height {
			// Offset from the center pixel to the edge of the window
			let offset = window_size / 2;

			// Quadrant A: Top-Left
			let q_a = (
				(x - offset).max(0)..(x + 1).min(image_width),
				(y - offset).max(0)..(y + 1).min(image_height),
			);

			// Quadrant B: Top-Right
			let q_b = (
				x.max(0)..(x + offset + 1).min(image_width),
				(y - offset).max(0)..(y + 1).min(image_height),
			);

			// Quadrant C: Bottom-Left
			let q_c = (
				(x - offset).max(0)..(x + 1).min(image_width),
				y.max(0)..(y + offset + 1).min(image_height),
			);

			// Quadrant D: Bottom-Right
			let q_d = (
				x.max(0)..(x + offset + 1).min(image_width),
				y.max(0)..(y + offset + 1).min(image_height),
			);

			let std_a = get_std_brightness_deviation_for_pixels(&luma8_image, &q_a);
			let std_b = get_std_brightness_deviation_for_pixels(&luma8_image, &q_b);
			let std_c = get_std_brightness_deviation_for_pixels(&luma8_image, &q_c);
			let std_d = get_std_brightness_deviation_for_pixels(&luma8_image, &q_d);

			let min_std = match [(q_a, std_a), (q_b, std_b), (q_c, std_c), (q_d, std_d)].iter().min_by(|a, b| {
				a.1.partial_cmp(&b.1).unwrap()
			}) {
				Some(x) => x.clone(),
				None => {
					eprintln!("???");
					exit(1);
				}
			};

			let mut pixels: Vec<[u8; 4]> = Vec::new();
			for z in min_std.0.0 {
				for w in min_std.0.1.clone() {
					pixels.push(image.get_pixel(z as u32, w as u32).0);
				}
			}

			let avg = rgba_average(pixels);

			new_image.put_pixel(x as u32, y as u32, Rgba(avg));
		}
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

/// Calculates the standard deviation for pixels from an image
/// within a given square.
fn get_std_brightness_deviation_for_pixels(
	image: &ImageBuffer<Luma<u8>, Vec<u8>>,
	ranges: &(Range<i32>, Range<i32>)
) -> u32 {
	let mut brightnesses: Vec<u32> = Vec::new();

	let x_range = ranges.0.clone();
	let y_range = ranges.1.clone();

	for x in x_range {
		for y in y_range.clone() {
			brightnesses.push(image.get_pixel(x as u32, y as u32).0[0] as u32);
		}
	}

	calculate_std_deviation(&brightnesses)
}

/// Calculates the average RGB color from the given colors.
fn rgba_average(colors: Vec<[u8; 4]>) -> [u8; 4] {
	let folded: [u32; 4] = colors.iter().fold([0, 0, 0, 0], |mut acc, color| {
		acc[0] += color[0] as u32;
		acc[1] += color[1] as u32;
		acc[2] += color[2] as u32;
		acc[3] += color[3] as u32;

		acc
	});

	return [
		(folded[0] / colors.len() as u32) as u8,
		(folded[1] / colors.len() as u32) as u8,
		(folded[2] / colors.len() as u32) as u8,
		(folded[3] / colors.len() as u32) as u8
	];
}

/// Calculates the variance for a vector of numbers.
fn calculate_variance(data: &Vec<u32>) -> u32 {
	let mean: u32 = data.iter().sum::<u32>() / data.len() as u32;
	let variance = data.iter().map(|val| {
		let diff = mean as i32 - (*val as i32);
		(diff * diff) as u32
	}).sum::<u32>() / data.len() as u32;

	variance
}

/// Calculates the standard deviation for a vector of numbers.
fn calculate_std_deviation(data: &Vec<u32>) -> u32 {
	let variance = calculate_variance(data);
	variance.isqrt()
}

/// Prints the help text for this effect.
#[cfg(not(target_arch = "wasm32"))]
fn print_help() {
	println!(r#"
Kuwahara Filter Effect
Applies a filter usually used for noise reduction which makes images look
like they were painted.

USAGE:
  effectengine-cli kuwahara <INPUT_PATH> <OUTPUT_PATH> [WINDOW_SIZE]

ARGUMENTS:
  <INPUT_PATH>     The path to an input image that should be processed.
  <OUTPUT_PATH>    The path where the resulting image should be saved.
                   Needs to include the filename.
  [WINDOW_SIZE]    Optional. How big the various "paint strokes" should
                   appear. Bigger numbers will mean bigger strokes and
                   less detail. (Default: 5)
  "#);
}
