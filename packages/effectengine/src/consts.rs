/// A list of all valid effects that can be used as the first CLI argument.
#[cfg(not(target_arch = "wasm32"))]
pub const VALID_EFFECTS: [&'static str; 13] = [
    "bayer-2",
    "bayer-4",
    "bayer-8",
    "bayer-16",
    "floyd-steinberg",
    "pixelate",
    "quantize",
    "pixel-sort",
    "kuwahara",
    "white-noise",
    "scanline",
    "bloom",
    "chromatic-aberration",
];

#[cfg(not(target_arch = "wasm32"))]
pub enum ValidEffect {
    Bayer2,
    Bayer4,
    Bayer8,
    Bayer16,
    FloydSteinberg,
    Pixelate,
    Quantize,
    PixelSort,
    Kuwahara,
    WhiteNoise,
    ScanLine,
    Bloom,
    ChromaticAberration,
}
