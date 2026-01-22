use wasm_bindgen::prelude::*;

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
#[wasm_bindgen(js_name = bayer4)]
pub fn effect(data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
	let bayer_4_matrix: Vec<Vec<u8>> = vec![
		vec![0, 128, 32, 160],
		vec![192, 64, 224, 96],
		vec![48, 176, 16, 144],
		vec![240, 112, 208, 80]
	];


		return apply_diffusion_kernel(
			data,
			bayer_4_matrix.len(),
			bayer_4_matrix.into_iter().flatten().collect::<Vec<u8>>(),
			width,
			height
		);
}
