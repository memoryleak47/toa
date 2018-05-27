use sfml::system::Vector2u;
use misc::Direction;

pub enum Command {
	Move { from: Vector2u, direction: Direction },
	NextTurn,
}
