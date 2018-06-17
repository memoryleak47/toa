use sfml::window::Key;
use sfml::system::{Vector2f, Vector2u};
use sfml::graphics::Color;

use player::Player;
use view::{View, default::DefaultViewGenerator};
use input::Input;
use world::{buildingmap::BUILDING_PLANS, World};
use command::Command;
use misc::{Direction, vector_if};

pub struct AiPlayer {
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
