use crate::*;

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct FoodClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Food;

impl ItemClassTrait for FoodClass {
	type Instance = Food;
	
	fn get_name() -> &'static str { "Food" }
	fn get_weight() -> u32 { 1 }
	fn build() -> Item {
		Item::Food(Food)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { None }
	fn stateless() -> bool { true }
}

impl ItemTrait for Food {
	type Class = FoodClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Food
	}
	fn damage(&mut self, _: Damage) -> bool { true }
}
