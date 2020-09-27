use crate::*;

lazy_static! {
	static ref RECIPE: [ItemClass; 3] = [ItemClass::Iron, ItemClass::Iron, ItemClass::Iron];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct LongSwordClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct LongSword {
	health: u32,
}

impl ItemClassTrait for LongSwordClass {
	type Instance = LongSword;

	fn get_name() -> &'static str { "LongSword" }
	fn get_weight() -> u32 { 40 }
	fn build() -> Item {
		Item::LongSword(LongSword { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for LongSword {
	type Class = LongSwordClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::LongSword
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn get_damage(&self) -> Damage { Damage(15) }
	fn aim(&self, v: Vec2f) -> Vec<Vec2i> {
		let t = melee_aim(v)[0];
		let orth = if t.x.abs() > t.y.abs() { Vec2i::new(0, 1) } else { Vec2i::new(1, 0) };

		iter::once(t)
			.chain(iter::once(t - orth))
			.chain(iter::once(t + orth))
			.collect()
	}
}
