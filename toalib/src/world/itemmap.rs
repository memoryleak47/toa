use crate::vec::Vec2u;
use crate::item::{Inventory, ItemClass};
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::world::damage::Damage;

const GROUND_DAMAGE: Damage = Damage(5);

impl World {
	pub fn tick_itemmap(&mut self) {
		self.damage_items_on_ground();
	}

	fn damage_items_on_ground(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				self.itemmap[index2d!(x, y)].damage(GROUND_DAMAGE);
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
	let mut x = init2d!(Inventory::new(), MAP_SIZE_X, MAP_SIZE_Y);

	x[index2d!(0, 0)].get_item_vec().push(ItemClass::Food.build()); // TODO remove

	x
}
