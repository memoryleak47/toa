use crate::vec::Vec2u;
use crate::item::Inventory;
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::damage::Damage;
use crate::world::buildingmap::BuildingClass;

const GROUND_DAMAGE: Damage = Damage(5);

impl World {
	pub fn tick_itemmap(&mut self) {
		self.damage_items_on_ground();
	}

	fn damage_items_on_ground(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let i = index2d!(x, y);
				if let Some(BuildingClass::Camp) = self.buildingmap[i].as_ref().map(|x| x.get_class()) { } else {
					self.itemmap[i].damage(GROUND_DAMAGE);
				}
			}
		}
	}

	pub fn get_inventory(&self, p: Vec2u) -> &Inventory {
		&self.itemmap[index2d!(p.x, p.y)]
	}

	pub fn get_inventory_mut(&mut self, p: Vec2u) -> &mut Inventory {
		&mut self.itemmap[index2d!(p.x, p.y)]
	}
}

pub fn new_itemmap() -> Vec<Inventory> {
	init2d!(Inventory::new(), MAP_SIZE_X, MAP_SIZE_Y)
}
