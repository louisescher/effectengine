use image::{DynamicImage, ImageBuffer, Rgba};

use crate::effects::bayer::apply_diffusion_kernel;

pub fn effect(image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let bayer_8_matrix: Vec<Vec<u8>> = vec![
		vec![0, 128, 32, 160, 8, 136, 40, 168],
		vec![192, 64, 224, 96, 200, 72, 232, 104],
		vec![48, 176, 16, 144, 56, 184, 24, 152],
		vec![240, 112, 208, 80, 248, 120, 216, 88],
		vec![12, 140, 44, 172, 4, 132, 36, 164],
		vec![204, 76, 236, 108, 196, 68, 228, 100],
		vec![60, 188, 28, 156, 52, 180, 20, 148],
		vec![252, 124, 220, 92, 244, 116, 212, 84]
	];

	return apply_diffusion_kernel(image, bayer_8_matrix);
}
