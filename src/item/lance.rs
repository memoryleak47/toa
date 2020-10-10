use crate::*;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Iron];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct LanceClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Lance {
	health: u32,
}

impl ItemClassTrait for LanceClass {
	type Instance = Lance;

	fn get_name() -> &'static str { "Lance" }
	fn get_weight() -> u32 { 15 }
	fn build() -> Item {
		Item::Lance(Lance { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for Lance {
	type Class = LanceClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Lance
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn get_damage(&self) -> Damage { Damage(12) }
	fn aim(&self, v: Vec2f) -> Vec<Vec2i> {
		let mut tiles = melee_aim(v);
		tiles.push(tiles[0] * 2);
		tiles
	}
}

