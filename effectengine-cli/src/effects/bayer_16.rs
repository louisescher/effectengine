use image::{DynamicImage, ImageBuffer, Rgba};

pub fn effect(_image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	println!("Using bayer-16...");

	let mut new_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(1, 1);

	new_image.put_pixel(0, 0, Rgba([0, 0, 0, 255]));

	return new_image;
}
