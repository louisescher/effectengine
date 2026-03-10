mod consts;
mod util;

pub mod effects {
    mod bayer;
    pub mod bayer_16;
    pub mod bayer_2;
    pub mod bayer_4;
    pub mod bayer_8;
    pub mod floyd_steinberg;
    pub mod kuwahara;
    pub mod pixel_sort;
    pub mod pixelate;
    pub mod quantize;
    pub mod white_noise;
}
