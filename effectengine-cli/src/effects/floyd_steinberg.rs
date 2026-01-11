use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::util::clamp_to_u8_space;

/// An implementation of the Floyd-Steinberg dithering algorithm. When a pixel's error is calculated, the
/// error is diffused down to other pixels with the following pattern (X is the current pixel, the numbers
/// are fractions of 16):
///
/// ```
/// |- - -|- - -|- - -|
/// |     |  x  |  7  |
/// |- - -|- - -|- - -|
/// |  3  |  5  |  1  |
/// |- - -|- - -|- - -|
/// ```
///
/// The algorithm does this by storing the diffused error in a one-dimensional array with the size equal
/// to the width of the image. Due to the divisor being 16, which is a multiple of two, bit-shifting can
/// be used for better performance.
pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let pixels = image.pixels();
	let image_width = image.width() as usize;

	let mut diffusion_array: Vec<i32> = Vec::new();
	let mut next_diff_err: i32 = 0;
	let mut current_row: usize = 0;
	let mut diff_array_for_row: Vec<i32> = Vec::new();

	// Pre-fill both arrays with zeroes so we can already use them on the first row.
	for _ in 0..image_width+1 {
		diffusion_array.push(0);
		diff_array_for_row.push(0);
	}

	let mut new_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(image_width as u32, image.height());

	for (i, pixel) in pixels.enumerate() {
		// Check if we're in a different row by now
		let row_check = i / image_width;
		if row_check > current_row {
			current_row += 1;
			next_diff_err = 0;

			// First, swap the arrays so we can work with the errors from previous rows
			std::mem::swap(&mut diff_array_for_row, &mut diffusion_array);

			// Next, clear the original diffusion array so it's good to work with again
			for v in &mut diffusion_array { *v = 0; }
		}

		let proper_index = i - (image_width * current_row);

		// Now we start computing the actual pixel errors
		let pixel_color = pixel_to_grayscale_value(pixel);

		// Factor in the errors from previous pixels
		let adjusted_pixel_color = clamp_to_u8_space(pixel_color + next_diff_err + diff_array_for_row[proper_index]);

		let pixel_error = if adjusted_pixel_color < 128 {
			new_image.put_pixel(pixel.0, pixel.1, Rgba([
				0,
				0,
				0,
				255
			]));

			adjusted_pixel_color
		} else {
			new_image.put_pixel(pixel.0, pixel.1, Rgba([
				255,
				255,
				255,
				255
			]));

			adjusted_pixel_color - 255
		};

		// The error for the next pixel to be processed.
		next_diff_err = pixel_error * 7 >> 4;

		// The errors for the next row of pixels, left to right.
		// In cases where we're on the first pixel of a row, we can't push to the bottom left pixel.
		if proper_index > 0 {
			diffusion_array[proper_index - 1] += pixel_error * 3 >> 4;
		}

		// This is the value for the pixel that's right below ours!
		diffusion_array[proper_index] += pixel_error * 5 >> 4;

		// Lastly, the pixel to the bottom right.
		// In cases where we're at the last pixel of a row, this pixel doesn't exist.
		if proper_index < image_width - 1 {
			diffusion_array[proper_index + 1] += pixel_error >> 4;
		}
	}

	return new_image;
}

/// Converts a pixel into a gray-scale version with a luminance calculation.
fn pixel_to_grayscale_value(pixel: (u32, u32, Rgba<u8>)) -> i32 {
	let pixel_rgb_info = pixel.2.0;
	let (r, g, b) = (pixel_rgb_info[0] as i32, pixel_rgb_info[1] as i32, pixel_rgb_info[2] as i32);

	return (r * 2126 + g * 7152 + b * 722) / 10000;
}
