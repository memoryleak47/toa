macro_rules! index2d {
	($width: expr, $height: expr) => {{
		use toalib::config::MAP_SIZE_X;

		($width as usize) + ($height as usize) * MAP_SIZE_X
	}}
}
