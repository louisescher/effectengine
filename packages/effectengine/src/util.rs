use std::{path::PathBuf, process::exit};

use image::{ImageFormat, ImageReader, Rgba};

use crate::consts::{VALID_EFFECTS, ValidEffect};

/// Checks if a given string is a hexadecimal color. The following requirements need to be
/// met for the string to be considered a hex color:
///
/// 1. The string needs to start with a `#`
/// 2. All other characters must either be digits (0 - 9) or a character from `a` to `f` (lower- or
/// uppercase)
pub fn is_hex_color(str: String) -> bool {
    // Has to start with a #
    if !str.starts_with('#') {
        return false;
    }

    // All other characters must not be # (needed for the next check to work)
    if str.clone().split_off(1).contains('#') {
        return false;
    }

    // All chars must be a #, a base 10 digit, or an ASCII-Character from a to f or A to F
    if !str.chars().all(|x| {
        x == '#'
		|| x.is_digit(10)
		|| x as u64 >= 65 && x as u64 <= 70 // Uppercase letter
		|| x as u64 >= 97 && x as u64 <= 102 // Lowercase letter
    }) {
        return false;
    }

    return true;
}

// Converts a hex color string to an RGBA color value that can be used, for example,
// when writing to an image.
pub fn hex_to_rgb(hex: String) -> Rgba<u8> {
    // Check if the string is a hex color first
    if !is_hex_color(hex.clone()) {
        eprintln!("Can't convert non-hex color string!");
        exit(1);
    }

    let mut chars = hex.chars();

    // Start with the second char to consume both it and the #, then always get the first one
    let r = format!(
        "{}{}",
        chars.nth(1).unwrap_or('0'),
        chars.nth(0).unwrap_or('0')
    );
    let g = format!(
        "{}{}",
        chars.nth(0).unwrap_or('0'),
        chars.nth(0).unwrap_or('0')
    );
    let b = format!(
        "{}{}",
        chars.nth(0).unwrap_or('0'),
        chars.nth(0).unwrap_or('0')
    );

    // Parse components as number from hex
    let color = Rgba([
        u8::from_str_radix(r.as_str(), 16).expect("Invalid hex string"),
        u8::from_str_radix(g.as_str(), 16).expect("Invalid hex string"),
        u8::from_str_radix(b.as_str(), 16).expect("Invalid hex string"),
        255,
    ]);

    return color;
}

/// Converts a pixel into a gray-scale version with a luminance calculation.
pub fn pixel_to_grayscale_value(pixel: (u32, u32, Rgba<u8>)) -> i32 {
    let pixel_rgb_info = pixel.2.0;
    let (r, g, b) = (
        pixel_rgb_info[0] as i32,
        pixel_rgb_info[1] as i32,
        pixel_rgb_info[2] as i32,
    );

    return (r * 2126 + g * 7152 + b * 722) / 10000;
}

/// A function to be used in an effect handler to check if a help flag is present.
#[cfg(not(target_arch = "wasm32"))]
pub fn subcommand_help_requested() -> bool {
    let collected_args: Vec<String> = std::env::args().collect();

    if collected_args.contains(&"--help".to_string()) || collected_args.contains(&"-h".to_string())
    {
        return true;
    }

    return false;
}

pub struct EffectEngineCliArgs {
    pub input_path: PathBuf,
    #[allow(dead_code)]
    pub output_path: PathBuf,
}

/// Validates a given effect and returns the enum associated with it.
/// Throws an error and exits if the effect does not exist.
#[allow(dead_code)]
pub fn validate_effect(effect: String) -> ValidEffect {
    match effect.as_str() {
        "bayer-2" => ValidEffect::Bayer2,
        "bayer-4" => ValidEffect::Bayer4,
        "bayer-8" => ValidEffect::Bayer8,
        "bayer-16" => ValidEffect::Bayer16,
        "floyd-steinberg" => ValidEffect::FloydSteinberg,
        "pixelate" => ValidEffect::Pixelate,
        "quantize" => ValidEffect::Quantize,
        "pixel-sort" => ValidEffect::PixelSort,
        "kuwahara" => ValidEffect::Kuwahara,
        "white-noise" => ValidEffect::WhiteNoise,
        "scanline" => ValidEffect::ScanLine,
        "bloom" => ValidEffect::Bloom,
        "chromatic-aberration" => ValidEffect::ChromaticAberration,
        _ => {
            eprintln!(
                "Invalid effect. Please provide one of the following effects as the first argument:"
            );

            for valid_effect in VALID_EFFECTS {
                eprintln!("    - {valid_effect}");
            }

            exit(64);
        }
    }
}

/// Gets the image input and output paths from the arguments passed to the CLI.
pub fn get_paths() -> EffectEngineCliArgs {
    let input_path = std::env::args()
        .nth(2)
        .expect("No input image given. Please provide the path to an input image.");
    let output_path = std::env::args()
        .nth(3)
        .expect("No output path given. Please provide the path to output the finished image to.");

    let args = EffectEngineCliArgs {
        input_path: PathBuf::from(input_path),
        output_path: PathBuf::from(output_path),
    };

    if !args.input_path.exists() {
        eprintln!("The given input path was not found.");
        exit(64);
    }

    if !args.input_path.is_file() {
        eprintln!("The given input path was not a file.");
        exit(64);
    }

    args
}

pub struct ImageData {
    pub format: ImageFormat,
    pub data: Vec<u8>,
}

pub fn read_image(path: PathBuf) -> ImageData {
    let image_reader_res = ImageReader::open(&path);

    let image_reader = match image_reader_res {
        Ok(_) => image_reader_res.unwrap(),
        Err(_) => {
            eprintln!("The image at the given path could not be read.");
            exit(64);
        }
    };

    let format = image_reader.format().unwrap();

    let encoded_input_image = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("The image at the given input path could not be read.");
            exit(64);
        }
    };

    let image_data = ImageData {
        format,
        data: encoded_input_image,
    };

    image_data
}
