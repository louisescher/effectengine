use std::process::exit;

use image::{ImageFormat, Rgba};

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
	let r = format!("{}{}", chars.nth(1).unwrap_or('0'), chars.nth(0).unwrap_or('0'));
	let g = format!("{}{}", chars.nth(0).unwrap_or('0'), chars.nth(0).unwrap_or('0'));
	let b = format!("{}{}", chars.nth(0).unwrap_or('0'), chars.nth(0).unwrap_or('0'));

	// Parse components as number from hex
	let color = Rgba([
		u8::from_str_radix(r.as_str(), 16).expect("Invalid hex string"),
		u8::from_str_radix(g.as_str(), 16).expect("Invalid hex string"),
		u8::from_str_radix(b.as_str(), 16).expect("Invalid hex string"),
		255
	]);

	return color;
}

/// Converts a pixel into a gray-scale version with a luminance calculation.
pub fn pixel_to_grayscale_value(pixel: (u32, u32, Rgba<u8>)) -> i32 {
	let pixel_rgb_info = pixel.2.0;
	let (r, g, b) = (pixel_rgb_info[0] as i32, pixel_rgb_info[1] as i32, pixel_rgb_info[2] as i32);

	return (r * 2126 + g * 7152 + b * 722) / 10000;
}

/// A function to be used in an effect handler to check if a help flag is present.
#[cfg(not(target_arch = "wasm32"))]
pub fn subcommand_help_requested() -> bool {
	let collected_args: Vec<String> = std::env::args().collect();

	if collected_args.contains(&"--help".to_string()) || collected_args.contains(&"-h".to_string()) {
		return true;
	}

	return false;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn image_format_to_number(input: ImageFormat) -> u8 {
	match input {
		ImageFormat::Avif => 0,
		ImageFormat::Bmp => 1,
		ImageFormat::Dds => 2,
		ImageFormat::Farbfeld => 3,
		ImageFormat::Gif => 4,
		ImageFormat::Hdr => 5,
		ImageFormat::Ico => 6,
		ImageFormat::Jpeg => 7,
		ImageFormat::OpenExr => 8,
		ImageFormat::Png => 9,
		ImageFormat::Pnm => 10,
		ImageFormat::Qoi => 11,
		ImageFormat::Tga => 12,
		ImageFormat::Tiff => 13,
		ImageFormat::WebP => 14,
		_ => panic!("Unknown format!")
	}
}

pub fn number_to_image_format(input: u8) -> ImageFormat {
	match input {
		0 => ImageFormat::Avif,
		1 => ImageFormat::Bmp,
		2 => ImageFormat::Dds,
		3 => ImageFormat::Farbfeld,
		4 => ImageFormat::Gif,
		5 => ImageFormat::Hdr,
		6 => ImageFormat::Ico,
		7 => ImageFormat::Jpeg,
		8 => ImageFormat::OpenExr,
		9 => ImageFormat::Png,
		10 => ImageFormat::Pnm,
		11 => ImageFormat::Qoi,
		12 => ImageFormat::Tga,
		13 => ImageFormat::Tiff,
		14 => ImageFormat::WebP,
		_ => panic!("Unknown format!")
	}
}
