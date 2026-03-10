mod consts;
mod effects;
mod util;

use std::path::PathBuf;
use std::process::exit;

use consts::*;
use effects::*;
use image::ImageReader;

use crate::util::image_format_to_number;

struct EffectEngineCliArgs {
    effect: ValidEffect,
    input_path: PathBuf,
    output_path: PathBuf,
}

fn main() {
    let collected_args: Vec<String> = std::env::args().collect();

    if collected_args.len() < 2
        || (collected_args.contains(&"--help".to_string()) && collected_args.len() < 4)
        || (collected_args.contains(&"-h".to_string()) && collected_args.len() < 4)
    {
        print_main_help();
        return;
    }

    let effect = std::env::args().nth(1).expect(
        format!(
            "No effect given. Please use one of the following effects: {}",
            VALID_EFFECTS.join(", ")
        )
        .as_str(),
    );
    let input_path = std::env::args()
        .nth(2)
        .expect("No input image given. Please provide the path to an input image.");
    let output_path = std::env::args()
        .nth(3)
        .expect("No output path given. Please provide the path to output the finished image to.");

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
            "scanline" => ValidEffect::ScanLine,
            "bloom" => ValidEffect::Bloom,
            "chromatic-abberation" => ValidEffect::ChromaticAbberation,
            _ => {
                eprintln!(
                    "Invalid effect. Please provide one of the following effects as the first argument:"
                );

                for valid_effect in VALID_EFFECTS {
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

    let input_image_reader_res = ImageReader::open(&args.input_path);

    let input_image_reader = match input_image_reader_res {
        Ok(_) => input_image_reader_res.unwrap(),
        Err(_) => {
            eprintln!("The image at the given input path could not be read.");
            exit(64);
        }
    };

    let format = input_image_reader.format().unwrap();

    let encoded_input_image = match std::fs::read(&args.input_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("The image at the given input path could not be read.");
            exit(64);
        }
    };

    let encoded_new_image = match args.effect {
        ValidEffect::Bayer2 => bayer_2::effect(encoded_input_image, image_format_to_number(format)),
        ValidEffect::Bayer4 => bayer_4::effect(encoded_input_image, image_format_to_number(format)),
        ValidEffect::Bayer8 => bayer_8::effect(encoded_input_image, image_format_to_number(format)),
        ValidEffect::Bayer16 => {
            bayer_16::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::FloydSteinberg => {
            floyd_steinberg::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::Pixelate => {
            pixelate::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::Quantize => {
            quantize::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::PixelSort => {
            pixel_sort::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::Kuwahara => {
            kuwahara::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::WhiteNoise => {
            white_noise::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::ScanLine => {
            scanline::effect(encoded_input_image, image_format_to_number(format))
        }
        ValidEffect::Bloom => scanline::effect(encoded_input_image, image_format_to_number(format)),
        ValidEffect::ChromaticAbberation => {
            scanline::effect(encoded_input_image, image_format_to_number(format))
        }
    };

    let new_image_res = std::fs::write(args.output_path, encoded_new_image);

    match new_image_res {
        Err(err) => {
            dbg!(err);
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
    println!(
        r#"
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
	"#,
        VALID_EFFECTS
            .map(|effect| format!("                        - {effect}"))
            .join("\n")
    );
}
