use sfml::audio::Music;

pub struct SoundState {
	music: Music
}

impl SoundState {
	pub fn new() -> Result<SoundState, String> {
		use toalib::misc::res_dir;

		let res_dir = res_dir();
		let res_dir_str = res_dir.to_str()
			.ok_or_else(|| "SoundState::new(): failed to get path-string for res-directory".to_string())?;
		let dir = format!("{}/sound/ambient01.ogg", res_dir_str);
		let mut music = Music::from_file(&dir)
			.ok_or_else(|| "SoundState::new(): failed loading music from file".to_string())?;
		music.set_looping(true);

		Ok(SoundState { music })
	}

	pub fn start(&mut self) {
		self.music.play();
	}
}
