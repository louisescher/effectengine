pub const VALID_EFFECTS: [&'static str; 3] = [
	"bayer-8",
	"bayer-16",
	"floyd-steinberg"
];

pub enum ValidEffect {
	Bayer8,
	Bayer16,
	FloydSteinberg
}
