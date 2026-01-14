use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::util::clamp_to_u8_space;

/// Applies a pixelation filter to an image by combining multiple pixels into bigger ones.
/// Calculates the average color to do so.
pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let image_width = image.width() as usize;
	let image_height = image.height();

	// How big the pixels in the final image should be
	let processed_pixel_size: usize = 8;

	let pixelized_width = (image_width) / processed_pixel_size;

	let mut new_pixels: Vec<Vec<(u32, u32, Rgba<u8>)>> = vec![Vec::new(); pixelized_width];

	let mut current_row: usize = 0;
	for (i, pixel) in image.pixels().enumerate() {
		let row_check = i / image_width;
		if row_check > current_row {
			current_row += 1;
		}

		// Check which "big pixel" we're in, push values to lower array
		// dbg!(i, processed_pixel_size, current_row, image_width);
		let chunk_idx = (i / processed_pixel_size) - (current_row * image_width / processed_pixel_size);

		new_pixels[chunk_idx].push(pixel);
	}

	let mut new_image = ImageBuffer::new(image_width as u32, image_height);

	current_row = 0;
	for (i, row) in new_pixels.iter().enumerate() {
		let row_check = i / image_width;
		if row_check > current_row {
			current_row += 1;
		}

		// Everything in here is one big pixel. The index * 8 is the width and the height. The big pixel has to be drawn to every real pixel inside that square.
		let pixel_idx = (i * 8) - (current_row * image_width);
		let adjusted_pixel_idx = pixel_idx + 8;

		let avg_rgba: (usize, usize, usize, usize) = row.iter().map(|x| (x.2.0[0], x.2.0[1], x.2.0[2], x.2.0[3])).fold((0, 0, 0, 0), |acc, x| {
			(
				acc.0 + x.0 as usize,
				acc.1 + x.1 as usize,
				acc.2 + x.2 as usize,
				acc.3 + x.3 as usize,
			)
		});

		for i in pixel_idx..adjusted_pixel_idx {
			for j in pixel_idx..adjusted_pixel_idx {
				// TODO: actually move through width and height
				// new_image.put_pixel(i as u32, j as u32, Rgba([
				// 	clamp_to_u8_space((avg_rgba.0 / row.len()) as i32) as u8,
				// 	clamp_to_u8_space((avg_rgba.1 / row.len()) as i32) as u8,
				// 	clamp_to_u8_space((avg_rgba.2 / row.len()) as i32) as u8,
				// 	clamp_to_u8_space((avg_rgba.3 / row.len()) as i32) as u8
				// ]));
			}
		}
	}

	return new_image;
}
