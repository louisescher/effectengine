mod consts;
mod effects;
mod util;

use std::path::{PathBuf};
use std::process::exit;

use consts::*;
use effects::*;
use image::{ColorType, DynamicImage, ImageBuffer, ImageReader};

struct EffectEngineCliArgs {
	effect: ValidEffect,
	input_path: PathBuf,
	output_path: PathBuf
}

fn main() {
	let collected_args: Vec<String> = std::env::args().collect();

	if
		collected_args.len() < 2
		|| (collected_args.contains(&"--help".to_string()) && collected_args.len() < 4)
		|| (collected_args.contains(&"-h".to_string()) && collected_args.len() < 4) {
		print_main_help();
		return;
	}

	let effect = std::env::args().nth(1).expect(format!("No effect given. Please use one of the following effects: {}", VALID_EFFECTS.join(", ")).as_str());
	let input_path = std::env::args().nth(2).expect("No input image given. Please provide the path to an input image.");
	let output_path = std::env::args().nth(3).expect("No output path given. Please provide the path to output the finished image to.");

	let args = EffectEngineCliArgs {
		effect: match effect.as_str() {
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
			_ => {
				eprintln!("Invalid effect. Please provide one of the following effects as the first argument:");

				for valid_effect in VALID_EFFECTS  {
					eprintln!("    - {valid_effect}");
				}

				exit(64);
			}
		},
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

	let input_image_reader_res = ImageReader::open(args.input_path);

	let input_image_reader = match input_image_reader_res {
		Ok(_) => input_image_reader_res.unwrap(),
		Err(_) => {
			eprintln!("The image at the given input path could not be read.");
			exit(64);
		}
	};

	let input_image_res = input_image_reader.decode();

	let input_image = match input_image_res {
		Ok(_) => input_image_res.unwrap(),
		Err(_) => {
			eprintln!("The image at the given input path could not be decoded.");
			exit(64);
		}
	};

	let raw_input_image = input_image.to_rgba8().into_raw();

	let raw_new_image = match args.effect {
		ValidEffect::Bayer2 => bayer_2::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::Bayer4 => bayer_4::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::Bayer8 => bayer_8::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::Bayer16 => bayer_16::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::FloydSteinberg => floyd_steinberg::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::Pixelate => pixelate::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::Quantize => quantize::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::PixelSort => pixel_sort::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::Kuwahara => kuwahara::effect(raw_input_image, input_image.width(), input_image.height()),
		ValidEffect::WhiteNoise => white_noise::effect(raw_input_image, input_image.width(), input_image.height())
	};

	let new_image = DynamicImage::ImageRgba8(ImageBuffer::from_raw(input_image.width(), input_image.height(), raw_new_image).unwrap());

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

	let new_image_res = formatted_new_image.save(args.output_path);

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

/// Prints the main help command.
fn print_main_help() {
	const VERSION: &str = env!("CARGO_PKG_VERSION");
	println!(r#"
EffectEngine CLI v{VERSION}
An image processing utility for various image effects.

USAGE:
    effectengine-cli <EFFECT> <INPUT_PATH> <OUTPUT_PATH> [SUBCOMMAND]

ARGUMENTS:
    <EFFECT>        The effect to use. Should be one of:
{}
    <INPUT_PATH>    The path to an input image that should be processed.
    <OUTPUT_PATH>   The path where the resulting image should be saved. Needs
                    to include the filename.
	"#, VALID_EFFECTS.map(|effect| format!("                        - {effect}")).join("\n"));
}
