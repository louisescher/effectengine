pub fn clamp_to_u8_space(val: i32) -> i32 {
	if val < 0 {
		0
	} else if val > 255 {
		255
	} else {
		val
	}
}
