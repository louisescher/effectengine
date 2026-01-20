use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::util::pixel_to_grayscale_value;

/// TODO: Sort in other directions
pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let image_width = image.width();
	let image_height = image.height();

	let mut new_image = ImageBuffer::new(image_width, image_height);

	let total_brightness = image.pixels().fold(0, |acc, pixel| {
		let grayscale = pixel_to_grayscale_value(pixel) as usize;

		acc + grayscale
	});

	let avg_brightness = total_brightness / (image_width as usize * image_height as usize);

	let mode = std::env::args().nth(4).or_else(|| Some(String::from("horizontal"))).unwrap();

	match mode.as_str() {
		"vertical" => {
			let (pixel_positions, pixels_to_be_sorted) = get_vertical_pixels_to_be_sorted(image, &mut new_image, avg_brightness);
			sort_pixels(&mut new_image, pixel_positions, pixels_to_be_sorted);
		},
		"both" => {
			let (mut pixel_positions, mut pixels_to_be_sorted) = get_horizontal_pixels_to_be_sorted(image, &mut new_image, avg_brightness);
			sort_pixels(&mut new_image, pixel_positions, pixels_to_be_sorted);

			let new_image_base = DynamicImage::ImageRgba8(new_image.clone());
			(pixel_positions, pixels_to_be_sorted) = get_vertical_pixels_to_be_sorted(&new_image_base, &mut new_image, avg_brightness);
			sort_pixels(&mut new_image, pixel_positions, pixels_to_be_sorted);
		},
		"horizontal" => {
			let (pixel_positions, pixels_to_be_sorted) = get_horizontal_pixels_to_be_sorted(image, &mut new_image, avg_brightness);
			sort_pixels(&mut new_image, pixel_positions, pixels_to_be_sorted);
		},
		_ => {
			let (pixel_positions, pixels_to_be_sorted) = get_horizontal_pixels_to_be_sorted(image, &mut new_image, avg_brightness);
			sort_pixels(&mut new_image, pixel_positions, pixels_to_be_sorted);
		}
	}

	return new_image;
}

fn sort_pixels(
	new_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	pixel_positions: Vec<Vec<Vec<(u32, u32)>>>,
	pixels_to_be_sorted: Vec<Vec<Vec<(Rgba<u8>, i32)>>>
) {
	let mut i = 0;
	for interval in pixels_to_be_sorted {
		let mut j = 0;
		for mut pixels in interval {
			pixels.sort_by(|a, b| {
				a.1.cmp(&b.1)
			});

			for (k, pixel) in pixels.iter().enumerate() {
				new_image.put_pixel(pixel_positions[i][j][k].0, pixel_positions[i][j][k].1, pixel.0);
			}

			j += 1;
		}

		i += 1;
	}
}

fn get_horizontal_pixels_to_be_sorted(
	image: &DynamicImage,
	new_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	avg_brightness: usize
) -> (
	Vec<Vec<Vec<(u32, u32)>>>,
	Vec<Vec<Vec<(Rgba<u8>, i32)>>>
) {
	let mut pixel_positions: Vec<Vec<Vec<(u32, u32)>>> = Vec::new();
	let mut pixels_to_be_sorted: Vec<Vec<Vec<(Rgba<u8>, i32)>>> = Vec::new();

	let mut current_row = 0;
	let mut interval = 0;

	for (i, pixel) in image.pixels().enumerate() {
		let row_check = i / image.width() as usize;
		if row_check > current_row {
			current_row += 1;
			interval = 0;
		}

		if pixel_positions.len() <= current_row {
			pixel_positions.push(Vec::new());
			pixels_to_be_sorted.push(Vec::new());
		}

		if pixel_positions[current_row].len() <= interval {
			pixel_positions[current_row].push(Vec::new());
			pixels_to_be_sorted[current_row].push(Vec::new());
		}

		let grayscale = pixel_to_grayscale_value(pixel);

		if grayscale > avg_brightness as i32 {
			pixel_positions[current_row][interval].push((
				pixel.0,
				pixel.1
			));

			pixels_to_be_sorted[current_row][interval].push((
				pixel.2,
				grayscale
			));
		} else {
			new_image.put_pixel(pixel.0, pixel.1, pixel.2);
			interval += 1;
		}
	}

	(pixel_positions, pixels_to_be_sorted)
}

fn get_vertical_pixels_to_be_sorted(
	image: &DynamicImage,
	new_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	avg_brightness: usize
) -> (
	Vec<Vec<Vec<(u32, u32)>>>,
	Vec<Vec<Vec<(Rgba<u8>, i32)>>>
) {
	let mut pixel_positions: Vec<Vec<Vec<(u32, u32)>>> = Vec::new();
	let mut pixels_to_be_sorted: Vec<Vec<Vec<(Rgba<u8>, i32)>>> = Vec::new();

	let width = image.width();
	let height = image.height();

	for x in 0..width {
		let mut column_positions = Vec::new();
		let mut column_pixels = Vec::new();

		let mut interval = 0;
		column_positions.push(Vec::new());
		column_pixels.push(Vec::new());

		for y in 0..height {
			let pixel = image.get_pixel(x, y);
			let grayscale = pixel_to_grayscale_value((x, y, pixel));

			if grayscale > avg_brightness as i32 {
				column_positions[interval].push((x, y));
				column_pixels[interval].push((pixel, grayscale));
			} else {
				new_image.put_pixel(x, y, pixel);

				if !column_positions[interval].is_empty() {
					interval += 1;
					column_positions.push(Vec::new());
					column_pixels.push(Vec::new());
				}
			}
		}
		pixel_positions.push(column_positions);
		pixels_to_be_sorted.push(column_pixels);
	}

	(pixel_positions, pixels_to_be_sorted)
}
