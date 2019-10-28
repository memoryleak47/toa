use crate::vec::Pos;
use crate::world::World;
use crate::damage::Damage;

const GROUND_DAMAGE: Damage = Damage(5);

impl World {
	pub fn tick_itemmap(&mut self) {
		self.damage_items_on_ground();
	}

	fn damage_items_on_ground(&mut self) {
		for p in Pos::iter_all() {
			if self.buildingmap.get(p)
					.map(|x| !x.get_class().prevents_item_despawn())
					.unwrap_or(true) {
				self.itemmap.get_mut(p).damage(GROUND_DAMAGE);
			}
		}
	}
}
