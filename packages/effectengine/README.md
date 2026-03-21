![EffectEngine](https://raw.githubusercontent.com/louisescher/effectengine/refs/heads/main/.github/assets/banner.png)

# effectengine

An image editing toolset written in Rust.

## Installation

**macOS / Linux: Homebrew (recommended)**

```sh
brew install louisescher/tap/effectengine
```

**macOS / Linux: install script**

```sh
curl -fsSL https://raw.githubusercontent.com/louisescher/effectengine/main/install.sh | sh
```

**Windows: Scoop**

```powershell
scoop bucket add louisescher https://github.com/louisescher/scoop-bucket
scoop install louisescher/effectengine
```

**Any platform — Cargo**

```sh
cargo install effectengine
```

Binaries are available via this repo's [GitHub Releases](https://github.com/louisescher/effectengine/releases).

## CLI usage

```
USAGE:
    effectengine-cli <EFFECT> <INPUT_PATH> <OUTPUT_PATH> [SUBCOMMAND]

ARGUMENTS:
    <EFFECT>        The effect to use.
    <INPUT_PATH>    The path to an input image that should be processed.
    <OUTPUT_PATH>   The path where the resulting image should be saved. Needs
                    to include the filename.
```

The entire CLI is self-documenting. You can pass `-h` or `--help` to any subcommand and get a full help text.

## Effects

Below you'll find a list of all effects this toolset supports, alongside the command to use.

| Effect                 | Description                                                                                                                          | Subcommand             |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ---------------------- |
| `bayer-2`              | Bayer dithering with a 2×2 diffusion matrix to convert images into high-contrast RGB-only pixels.                                    | `bayer-2`              |
| `bayer-4`              | Bayer dithering with a 4×4 diffusion matrix for finer thresholding and smoother gradients than the 2×2 version.                      | `bayer-4`              |
| `bayer-8`              | Bayer dithering with an 8×8 diffusion matrix, offering even finer dithering and richer gradients.                                    | `bayer-8`              |
| `bayer-16`             | Bayer dithering using a 16×16 matrix for the most detailed and subtle dithering patterns.                                            | `bayer-16`             |
| `floyd-steinberg`      | Floyd-Steinberg error-diffusion dithering that approximates the image using only dark and light pixels, with optional custom colors. | `floyd-steinberg`      |
| `pixelate`             | Pixelates the image by grouping pixels into larger blocks and using each block's average color.                                      | `pixelate`             |
| `quantize`             | Quantizes colors to a provided palette (from a path or hex codes), replacing each pixel with the nearest palette color.              | `quantize`             |
| `pixel-sort`           | Sorts pixels above the average brightness in horizontal, vertical, or both directions to create a tear-like distortion.              | `pixel-sort`           |
| `kuwahara`             | Applies the Kuwahara filter to reduce noise and produce a painterly, brush-stroke look via quadrant variance analysis.               | `kuwahara`             |
| `white-noise`          | Overlays the image with adjustable white noise, blending random values at a given opacity.                                           | `white-noise`          |
| `scanline`             | Produces a scanline effect reminiscent of CRT displays by dimming every other row.                                                   | `scanline`             |
| `bloom`                | Blurs the image with a gaussian kernel and additively blends it back to create a bloom/glow effect, with configurable sigma.         | `bloom`                |
| `chromatic-aberration` | Simulates lens chromatic fringing by offsetting the RGB channels radially, with adjustable strength.                                 | `chromatic-aberration` |
