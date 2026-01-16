use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// TODO: Pixel Sort
pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let image_width = image.width();
	let image_height = image.height();

	let mut new_image = ImageBuffer::new(image_width, image_height);

	return new_image;
}
