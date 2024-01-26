use crate::*;

pub struct SoundState {
	music: Music<'static>
}

impl SoundState {
	pub fn new() -> Result<SoundState, String> {
		let mut music = Music::from_file(&resource("sound/full.ogg"))
			.ok_or_else(|| "SoundState::new(): failed loading music from file".to_string())?;
		music.set_looping(true);

		Ok(SoundState { music })
	}

	pub fn start(&mut self) {
		self.music.play();
	}
}
