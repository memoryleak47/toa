use sfml::system::Vector2u;

use item::{Item, Inventory};
use world::{World, MAP_SIZE_X, MAP_SIZE_Y};

impl World {
	pub fn tick_itemmap(&mut self) {
		self.damage_items_on_ground();
	}

	fn damage_items_on_ground(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				for item in self.itemmap[x][y].as_mut() {
					item.damage();
				}
				// TODO remove dead items
			}
		}
	}

	pub fn get_inventory(&self, p: Vector2u) -> &Inventory {
		&self.itemmap[p.x as usize][p.y as usize]
	}

	pub fn get_inventory_mut(&mut self, p: Vector2u) -> &mut Inventory {
		&mut self.itemmap[p.x as usize][p.y as usize]
	}
}

pub fn new_itemmap() -> [[Inventory; MAP_SIZE_Y]; MAP_SIZE_X] {
	init2d!(Inventory::new(), MAP_SIZE_X, MAP_SIZE_Y)
}
