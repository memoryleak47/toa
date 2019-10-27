use crate::vec::Pos;
use crate::item::Inventory;
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::damage::Damage;

const GROUND_DAMAGE: Damage = Damage(5);

impl World {
	pub fn tick_itemmap(&mut self) {
		self.damage_items_on_ground();
	}

	fn damage_items_on_ground(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let i = index2d!(x, y);
				if self.buildingmap[i].as_ref()
						.map(|x| !x.get_class().prevents_item_despawn())
						.unwrap_or(true) {
					self.itemmap[i].damage(GROUND_DAMAGE);
				}
			}
		}
	}

	pub fn get_inventory(&self, p: Pos) -> &Inventory {
		&self.itemmap[index2d!(p.x, p.y)]
	}

	pub fn get_inventory_mut(&mut self, p: Pos) -> &mut Inventory {
		&mut self.itemmap[index2d!(p.x, p.y)]
	}
}

pub fn new_itemmap() -> Vec<Inventory> {
	init2d!(Inventory::new(), MAP_SIZE_X, MAP_SIZE_Y)
}
