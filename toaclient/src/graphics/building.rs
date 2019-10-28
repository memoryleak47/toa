use sfml::graphics::Color;

use toalib::world::buildingmap::Building;
use toalib::team::{PlayerID, COLORS};

use crate::graphics::TextureId;

pub fn get_colored_texture_id(building: &Building) -> (TextureId, Color) {
	let conv_color = |(r, g, b)| Color::rgb(r, g, b);
	let color_of = |pid: PlayerID| conv_color(COLORS[pid.0]);
	let x = match building {
		Building::Spawner(spawner) => {
			return (TextureId::SpawnerBuilding, color_of(spawner.get_player_id()))
		},
		Building::Farm(_) => TextureId::FarmBuilding,
		Building::Camp(_) => TextureId::CampBuilding,
		Building::Sawmill(_) => TextureId::SawmillBuilding,
		Building::StoneMine(_) => TextureId::StoneMineBuilding,
		Building::IronMine(_) => TextureId::IronMineBuilding,
		Building::Workshop(_) => TextureId::WorkshopBuilding,
		Building::Castle(_) => TextureId::CastleBuilding,
		Building::WoodWall(_) => TextureId::WoodWallBuilding,
		Building::StoneWall(_) => TextureId::StoneWallBuilding,
		Building::Street(_) => TextureId::StreetBuilding,
	};
	(x, Color::WHITE)
}

