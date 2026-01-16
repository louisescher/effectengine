use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// Applies a pixelation filter to an image by combining multiple pixels into bigger ones.
/// Calculates the average color to do so.
pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let image_width = image.width();
	let image_height = image.height();

	// How big the pixels in the final image should be
	let processed_pixel_size: u32 = 128;

	let mut new_image = ImageBuffer::new(image_width, image_height);

	for i in (0..image_width).step_by(processed_pixel_size as usize) {
		for j in (0..image_height).step_by(processed_pixel_size as usize) {
			let mut r_sum: u64 = 0;
			let mut g_sum: u64 = 0;
			let mut b_sum: u64 = 0;
			let mut a_sum: u64 = 0;
			let mut count: u64 = 0;

			let x_end: u32 = (i + processed_pixel_size).min(image_width);
			let y_end: u32 = (j + processed_pixel_size).min(image_height);

			for k in i..x_end {
				for l in j..y_end {
					let pixel = image.get_pixel(k, l);

					r_sum += pixel.0[0] as u64;
					g_sum += pixel.0[1] as u64;
					b_sum += pixel.0[2] as u64;
					a_sum += pixel.0[3] as u64;
					count += 1;
				}
			}

			if count > 0 {
				let color = Rgba([
					(r_sum / count) as u8,
					(g_sum / count) as u8,
					(b_sum / count) as u8,
					(a_sum / count) as u8
				]);

				for k in i..x_end {
					for l in j..y_end {
						new_image.put_pixel(k, l, color);
					}
				}
			}
		}
	}

	return new_image;
}
