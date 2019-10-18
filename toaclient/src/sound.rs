use sfml::audio::Music;

pub struct SoundState {
	music: Music
}

impl SoundState {
	pub fn new() -> Result<SoundState, String> {
		use crate::misc::resource;

		let mut music = Music::from_file(&resource("sound/full.ogg"))
			.ok_or_else(|| "SoundState::new(): failed loading music from file".to_string())?;
		music.set_looping(true);

		Ok(SoundState { music })
	}

	pub fn start(&mut self) {
		self.music.play();
	}
}
