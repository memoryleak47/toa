use sfml::system::Vector2u;

use item::Inventory;
use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use world::damage::Damage;

const GROUND_DAMAGE: Damage = Damage(5);

impl World {
	pub fn tick_itemmap(&mut self) {
		self.damage_items_on_ground();
	}

	fn damage_items_on_ground(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				self.itemmap[x][y].damage(GROUND_DAMAGE);
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
	use ::item::ItemClass;
	let mut x = init2d!(Inventory::new(), MAP_SIZE_X, MAP_SIZE_Y);

	x[0][0].get_item_vec().push(::item::food::FoodClass.get_ref().build()); // TODO remove

	x
}
