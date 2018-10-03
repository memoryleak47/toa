use font_kit::source::SystemSource;
use font_kit::handle::Handle;
use font_kit::family_name::FamilyName;
use sfml::graphics::Font;

use std::path::PathBuf;

static FAMILIES: [FamilyName; 1] = [FamilyName::SansSerif];

fn get_font_path() -> PathBuf {
	let source = SystemSource::new();
	let props = Default::default();
	if let Handle::Path { path, .. } = source.select_best_match(&FAMILIES, &props).unwrap() {
		path
	} else {
		panic!("font failure!")
	}
}

pub fn get_font() -> Font {
	let path = get_font_path();
	Font::from_file(path.to_str().unwrap()).unwrap()
}
