use crate::player::Player;
use crate::view::{View, default::DefaultViewGenerator};
use crate::input::Input;
use crate::command::Command;
use crate::world::World;

pub struct AiPlayer {
	#[allow(dead_code)]
	player_id: u32,
	view_generator: DefaultViewGenerator
}

impl AiPlayer {
	pub fn new(player_id: u32) -> AiPlayer {
		AiPlayer {
			player_id,
			view_generator: DefaultViewGenerator::new(player_id),
		}
	}

}

impl Player for AiPlayer {
	fn tick(&mut self, _: &World, input: &Input) -> Option<Command> {
		self.view_generator.tick(input);
		None
	}

	fn get_view(&self, w: &World) -> View {
		self.view_generator.get_view(w)
	}
}
