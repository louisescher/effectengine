pub const VALID_EFFECTS: [&'static str; 6] = [
	"bayer-2",
	"bayer-4",
	"bayer-8",
	"bayer-16",
	"floyd-steinberg",
	"pixelate"
];

pub enum ValidEffect {
	Bayer2,
	Bayer4,
	Bayer8,
	Bayer16,
	FloydSteinberg,
	Pixelate
}
