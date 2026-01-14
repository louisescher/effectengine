use image::{DynamicImage, ImageBuffer, Rgba};

use crate::effects::bayer::apply_diffusion_kernel;

pub fn effect(image: &mut DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let bayer_8_matrix: Vec<Vec<u8>> = vec![
		vec![0, 128],
		vec![192, 64]
	];

	return apply_diffusion_kernel(image, bayer_8_matrix);
}
