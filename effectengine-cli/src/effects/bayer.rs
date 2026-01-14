use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// A function that takes in an image and a diffusion matrix. It applies ordered
/// dithering to the image based on the given matrix.
pub fn apply_diffusion_kernel(
	image: &mut DynamicImage,
	kernel: Vec<Vec<u8>>
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let kernel_height = kernel.len() as u32;
	let kernel_width = kernel[0].len() as u32;
	let (width, height) = image.dimensions();

	let mut new_image = ImageBuffer::new(width, height);

	for y in 0..height {
		for x in 0..width {
			let pixel = image.get_pixel(x, y);

			let threshold = kernel[(y % kernel_height) as usize][(x % kernel_width) as usize] as f32;

			let r = if (pixel[0] as f32) > threshold { 255 } else { 0 };
			let g = if (pixel[1] as f32) > threshold { 255 } else { 0 };
			let b = if (pixel[2] as f32) > threshold { 255 } else { 0 };

			new_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
		}
	}

	return new_image;
}
