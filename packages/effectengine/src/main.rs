mod consts;
mod effects;
mod util;

use std::process::exit;

use consts::*;
use effects::*;

use crate::util::{get_paths, validate_effect};

fn main() {
    let collected_args: Vec<String> = std::env::args().collect();

    if collected_args.len() < 2
        || (collected_args.contains(&"--help".to_string()) && collected_args.len() < 3)
        || (collected_args.contains(&"-h".to_string()) && collected_args.len() < 3)
    {
        print_main_help();
        return;
    }

    let effect_arg = std::env::args().nth(1).expect(
        format!(
            "No effect given. Please use one of the following effects: {}",
            VALID_EFFECTS.join(", ")
        )
        .as_str(),
    );

    let effect = validate_effect(effect_arg);

    let encoded_new_image = match effect {
        ValidEffect::Bayer2 => bayer_2::effect(),
        ValidEffect::Bayer4 => bayer_4::effect(),
        ValidEffect::Bayer8 => bayer_8::effect(),
        ValidEffect::Bayer16 => bayer_16::effect(),
        ValidEffect::FloydSteinberg => floyd_steinberg::effect(),
        ValidEffect::Pixelate => pixelate::effect(),
        ValidEffect::Quantize => quantize::effect(),
        ValidEffect::PixelSort => pixel_sort::effect(),
        ValidEffect::Kuwahara => kuwahara::effect(),
        ValidEffect::WhiteNoise => white_noise::effect(),
        ValidEffect::ScanLine => scanline::effect(),
        ValidEffect::Bloom => bloom::effect(),
        ValidEffect::ChromaticAberration => chromatic_aberration::effect(),
    };

    let args = get_paths();

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
    effectengine <EFFECT> <INPUT_PATH> <OUTPUT_PATH> [SUBCOMMAND]

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
