/// A list of all valid effects that can be used as the first CLI argument.
pub const VALID_EFFECTS: [&'static str; 8] = [
	"bayer-2",
	"bayer-4",
	"bayer-8",
	"bayer-16",
	"floyd-steinberg",
	"pixelate",
	"quantize",
	"pixel-sort"
];

pub enum ValidEffect {
	Bayer2,
	Bayer4,
	Bayer8,
	Bayer16,
	FloydSteinberg,
	Pixelate,
	Quantize,
	PixelSort,
	Kuwahara
}
