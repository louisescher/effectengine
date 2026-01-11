use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};

use crate::util::clamp_to_u8_space;

/// A function that takes in an image and a diffusion matrix. It applies ordered
/// dithering to the image based on the given matrix.
pub fn apply_diffusion_kernel(
	image: &mut DynamicImage,
	kernel: Vec<Vec<u8>>
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let kernel_height = kernel.len() as u32;
	let kernel_width = kernel[0].len() as u32;
	let kernel_radius = (((kernel_width - 1) / 2) as f32).floor();
	let width = image.width();
	let height = image.height();

	let mut new_image = ImageBuffer::new(width, height);

	for y in 0..height {
		for x in 0..width {
			let pixel = image.get_pixel(x, y);
			let x_index = (x % kernel_height) as usize;
			let y_index = (y % kernel_height) as usize;
			let matrix_entry = kernel[x_index][y_index] as i32;
			let (r, g, b) = (pixel.0[0] as i32, pixel.0[1] as i32, pixel.0[2] as i32);

			let color_space_spread: i32 = 17;

			let (new_r, new_g, new_b) = (
				r + color_space_spread + (matrix_entry - 1/2),
				g + color_space_spread + (matrix_entry - 1/2),
				b + color_space_spread + (matrix_entry - 1/2)
			);

			new_image.put_pixel(x, y, Rgba([
				clamp_to_u8_space(new_r) as u8,
				clamp_to_u8_space(new_g) as u8,
				clamp_to_u8_space(new_b) as u8,
				pixel.0[3] as u8
			]));

			let (error_r, error_g, error_b) = (
				(r as i32 - new_r as i32),
				(g as i32 - new_g as i32),
				(b as i32 - new_b as i32)
			);

			for diff_x in 0..kernel_width {
				for diff_y in 0..kernel_height {
					let diff_weight = kernel[diff_y as usize][diff_x as usize] as i32;

					if diff_weight == 0 { continue };

					let neighbour_x = x as i32 + diff_x as i32 - kernel_radius as i32;
					let neighbour_y = y as i32 + diff_y as i32;

					if neighbour_x >= 0 && neighbour_y >= 0 && (neighbour_x as u32) < width && (neighbour_y as u32) < height {
						let nx = neighbour_x as u32;
						let ny = neighbour_y as u32;
						let neighbour_pixel = image.get_pixel(nx, ny);

						image.put_pixel(x, y, Rgba([
							clamp_to_u8_space(neighbour_pixel.0[0] as i32 + error_r * diff_weight) as u8,
							clamp_to_u8_space(neighbour_pixel.0[1] as i32 + error_g * diff_weight) as u8,
							clamp_to_u8_space(neighbour_pixel.0[2] as i32 + error_b * diff_weight) as u8,
							neighbour_pixel.0[3],
						]));
					}
				}
			}
		}
	}

	return new_image;
}
