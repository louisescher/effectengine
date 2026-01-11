use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::util::clamp_to_u8_space;

/// A function that takes in an image and a diffusion matrix. It applies ordered
/// dithering to the image based on the given matrix.
pub fn apply_diffusion_kernel(
	image: &DynamicImage,
	kernel: Vec<Vec<u8>>
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let kernel_height = kernel.len() as u32;
	let kernel_width = kernel[0].len() as u32;
	let kernel_radius = (((kernel_width - 1) / 2) as f32).floor();
	let pixels = image.pixels();
	let width = image.width();
	let height = image.height();

	let mut new_image = ImageBuffer::new(width, height);

	for (i, pixel) in pixels.enumerate() {
		let x_index = (pixel.0 % kernel_height) as usize;
		let y_index = (pixel.1 % kernel_height) as usize;
		let matrix_entry = kernel[x_index][y_index];
		let (r, g, b) = (pixel.2.0[0], pixel.2.0[1], pixel.2.0[2]);

		let color_space_spread: u8 = 17;

		let (new_r, new_g, new_b) = (
			r + color_space_spread + (matrix_entry - 1/2),
			g + color_space_spread + (matrix_entry - 1/2),
			b + color_space_spread + (matrix_entry - 1/2)
		);

		let (x, y) = (pixel.0, pixel.1);

		new_image.put_pixel(x, y, Rgba([new_r, new_g, new_b, pixel.2.0[4]]));

		let (error_r, error_g, error_b) = (
			(r - new_r) as i32,
			(g - new_g) as i32,
			(b - new_b) as i32
		);

		for diff_x in 0..kernel_width {
			for diff_y in 0..kernel_height {
				let diff_weight = kernel[diff_y as usize][diff_x as usize];

				if diff_weight == 0 { continue };

				let neighbour_x = x + diff_x - kernel_radius as u32;
				let neighbour_y = y + diff_y;

				// TODO: Fix (https://github.com/delucis/sweetcorn/blob/d962bd93727e0d60ba69a22e839637351545fddd/packages/sweetcorn/src/processors.ts#L55-L59)
				// https://en.wikipedia.org/wiki/Ordered_dithering
				// if neighbour_x >= 0 && neighbour_y >= 0 && neighbour_x < width && neighbour_y < height {
				// 	let next_pixel = pixels.nth(i + 2);
				// 	pixels[neighbour_index] = clamp_to_u8_space(pixels[neighbour_index] + error * diffusionWeight);
				// }
			}
		}
	}

	return ImageBuffer::new(1, 1);
}
