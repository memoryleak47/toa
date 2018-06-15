use std::{ptr, mem};

use sfml::system::Vector2u;

use item::{Item, Inventory};
use world::{World, MAP_SIZE_X, MAP_SIZE_Y};

impl World {
	pub fn drop_item(&mut self, item: Item, here: Vector2u) {
		if let Some(ref mut x) = self.get_unit_mut(here) {
			x.inventory.push(item);
			return;
		}

		self.get_inventory_mut(here).push(item);
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
