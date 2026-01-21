use image::{DynamicImage, ImageBuffer, Rgba};

use crate::effects::bayer::apply_diffusion_kernel;

/// Applies Bayer dithering to a given image with the following 4x4 Bayer matrix:
///
/// ```
/// | --- | --- | --- | --- |
/// |  0  | 128 | 32  | 160 |
/// | --- | --- | --- | --- |
/// | 192 | 64  | 224 | 65  |
/// | --- | --- | --- | --- |
/// | 48  | 176 | 16  | 144 |
/// | --- | --- | --- | --- |
/// | 240 | 112 | 208 | 80  |
/// | --- | --- | --- | --- |
/// ```
pub fn effect(image: &mut DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let bayer_4_matrix: Vec<Vec<u8>> = vec![
		vec![0, 128, 32, 160],
		vec![192, 64, 224, 96],
		vec![48, 176, 16, 144],
		vec![240, 112, 208, 80]
	];

	return apply_diffusion_kernel(image, bayer_4_matrix);
}
