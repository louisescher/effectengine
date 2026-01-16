use std::process::exit;

use image::Rgba;

pub fn clamp_to_u8_space(val: i32) -> i32 {
	if val < 0 {
		0
	} else if val > 255 {
		255
	} else {
		val
	}
}

pub fn is_hex_color(str: String) -> bool {
	if !str.starts_with('#') {
		return false;
	}

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

pub fn hex_to_rgb(hex: String) -> Rgba<u8> {
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
