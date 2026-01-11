mod consts;
mod effects;
mod util;

use std::path::{PathBuf};
use std::process::exit;

use consts::*;
use effects::*;
use image::{ColorType, DynamicImage, ImageReader};

struct EffectEngineCliArgs {
	effect: ValidEffect,
	input_path: PathBuf,
	output_path: Option<PathBuf>
}

fn main() {
	let effect = std::env::args().nth(1).expect(format!("No effect given. Please use one of the following effects: {}", VALID_EFFECTS.join(", ")).as_str());
	let input_path = std::env::args().nth(2).expect("No input image given. Please provide the path to an input image.");
	let output_path = std::env::args().nth(3);

	let args = EffectEngineCliArgs {
		effect: match effect.as_str() {
			"bayer-8" => ValidEffect::Bayer8,
			"bayer-16" => ValidEffect::Bayer16,
			"floyd-steinberg" => ValidEffect::FloydSteinberg,
			_ => {
				eprintln!("Invalid effect. Please provide one of the following effects as the first argument:");

				for valid_effect in VALID_EFFECTS  {
					eprintln!("    - {valid_effect}");
				}

				exit(64);
			}
		},
		input_path: PathBuf::from(input_path),
		output_path: match output_path {
			Some(_) => Some(PathBuf::from(&output_path.unwrap())),
			None => None
		}
	};

	if !args.input_path.exists() {
		eprintln!("The given input path was not found.");
		exit(64);
	}

	if !args.input_path.is_file() {
		eprintln!("The given input path was not a file.");
		exit(64);
	}

	let input_image_reader_res = ImageReader::open(args.input_path);

	let input_image_reader = match input_image_reader_res {
		Ok(_) => input_image_reader_res.unwrap(),
		Err(_) => {
			eprintln!("The image at the given input path could not be read.");
			exit(64);
		}
	};

	let input_image_res = input_image_reader.decode();

	let mut input_image = match input_image_res {
		Ok(_) => input_image_res.unwrap(),
		Err(_) => {
			eprintln!("The image at the given input path could not be decoded.");
			exit(64);
		}
	};

	let new_image = DynamicImage::ImageRgba8(match args.effect {
		ValidEffect::Bayer8 => bayer_8::effect(&mut input_image),
		ValidEffect::Bayer16 => bayer_16::effect(&input_image),
		ValidEffect::FloydSteinberg => floyd_steinberg::effect(&input_image),
	});

	let formatted_new_image = match input_image.color() {
		ColorType::Rgb8 => DynamicImage::ImageRgb8(new_image.into_rgb8()),
		ColorType::Rgba8 => new_image,
		ColorType::L8 => DynamicImage::ImageLuma8(new_image.into_luma8()),
		ColorType::La8 => DynamicImage::ImageLumaA8(new_image.into_luma_alpha8()),
		ColorType::Rgb16 => DynamicImage::ImageRgb16(new_image.into_rgb16()),
		ColorType::Rgba16 => DynamicImage::ImageRgba16(new_image.into_rgba16()),
		_ => {
			// Fallback or handle specific formats like BGR if necessary
			new_image
		}
	};

	if !args.output_path.is_some() {
		return;
	}

	let new_image_res = formatted_new_image.save(args.output_path.unwrap());

	match new_image_res {
		Err(image_error) => {
			dbg!(image_error);
			exit(1);
		}
		_ => {
			exit(0);
		}
	}
}
