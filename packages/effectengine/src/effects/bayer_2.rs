use wasm_bindgen::prelude::*;

use crate::{effects::bayer::apply_diffusion_kernel, util::number_to_image_format};

/// Applies Bayer dithering to a given image with the following 2x2 Bayer matrix:
///
/// ```
/// | --- | --- |
/// |  0  | 128 |
/// | --- | --- |
/// | 192 | 64  |
/// | --- | --- |
/// ```
#[wasm_bindgen(js_name = bayer2)]
pub fn effect(data: Vec<u8>, image_format: u8) -> Vec<u8> {
    let bayer_2_matrix: Vec<Vec<u8>> = vec![vec![0, 128], vec![192, 64]];

    return apply_diffusion_kernel(
        data,
        bayer_2_matrix.len(),
        bayer_2_matrix.into_iter().flatten().collect::<Vec<u8>>(),
        number_to_image_format(image_format),
    );
}
