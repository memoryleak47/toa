mod food;
pub use food::*;

mod wood;
pub use wood::*;

mod wood_sword;
pub use wood_sword::*;

mod stone;
pub use stone::*;

mod iron;
pub use iron::*;

mod iron_sword;
pub use iron_sword::*;

mod wood_bow;
pub use wood_bow::*;

mod settlement_kit;
pub use settlement_kit::*;

mod long_sword;
pub use long_sword::*;

mod lance;
pub use lance::*;

use std::slice;

use crate::*;

pub trait ItemTrait {
	type Class: ItemClassTrait + Sized;

	fn get_class(&self) -> ItemClass;

	fn damage(&mut self, damage: Damage) -> bool; // returns whether the item got destroyed
	fn get_damage(&self) -> Damage { Damage(1) }
	// relative mouse position (returning vec![mouse.to_i()]) yields the tile pointed by the mouse
	fn aim(&self, mouse: Vec2f) -> Vec<Vec2i> { melee_aim(mouse) }
	fn is_execable(&self, _p: Pos, _w: &World) -> bool { false }
	fn exec(&self, _p: Pos, _w: &mut World) { panic!("default ItemTrait::exec() was called!"); }
}

pub trait ItemClassTrait {
	type Instance: ItemTrait + Sized;

	fn get_name() -> &'static str;
	fn get_weight() -> u32;
	fn build() -> Item;
	fn get_recipe() -> Option<&'static [ItemClass]>;
	fn stateless() -> bool;
}

macro_rules! setup {
	($($x:ident),*) => {

		lazy_static! {
			pub static ref ITEM_CLASSES: Vec<ItemClass> = vec![ $( ItemClass::$x ),* ];
			pub static ref CRAFTABLE_ITEM_CLASSES: Vec<ItemClass> = ITEM_CLASSES.iter()
				.filter(|x| x.get_recipe().is_some())
				.cloned()
				.collect();
		}

		#[derive(Serialize, Deserialize)]
		#[derive(Clone)]
		pub enum Item {
			$(  $x($x)  ),*
		}

		#[derive(Serialize, Deserialize)]
		#[derive(PartialEq, Eq, Copy, Clone)]
		pub enum ItemClass {
			$( $x ),*
		}

		impl Item {
			pub fn get_class(&self) -> ItemClass						{ match self { $( Item::$x(a) => a.get_class() ),* } }
			pub fn damage(&mut self, damage: Damage) -> bool	{ match self { $( Item::$x(a) => a.damage(damage) ),* } }
			pub fn get_damage(&self) -> Damage							{ match self { $( Item::$x(a) => a.get_damage() ),* } }
			// relative mouse position (returning vec![mouse.to_i()]) yields the tile pointed by the mouse
			pub fn aim(&self, mouse: Vec2f) -> Vec<Vec2i>					{ match self { $( Item::$x(a) => a.aim(mouse) ),* } }
			pub fn is_execable(&self, p: Pos, w: &World) -> bool		{ match self { $( Item::$x(a) => a.is_execable(p, w) ),* } }
			pub fn exec(&self, p: Pos, w: &mut World)					{ match self { $( Item::$x(a) => a.exec(p, w) ),* } }
		}

		impl ItemClass {
			pub fn get_name(&self) -> &'static str						{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::get_name() ),* } }
			pub fn get_weight(&self) -> u32								{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::get_weight() ),* } }
			pub fn build(&self) -> Item									{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::build() ),* } }
			pub fn get_recipe(&self) -> Option<&'static [ItemClass]> 	{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::get_recipe() ),* } }
			pub fn stateless(&self) -> bool							 	{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::stateless() ),* } }
		}
	};
}

setup!(Food, Wood, WoodSword, Stone, Iron, IronSword, WoodBow, SettlementKit, LongSword, Lance);

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
	items: Vec<Item>,
}

impl Inventory {
	pub fn new() -> Inventory {
		Inventory { items: Vec::new() }
	}

	pub fn push(&mut self, item: Item) {
		self.items.push(item);
	}

	pub fn remove(&mut self, index: usize) -> Item {
		self.items.remove(index)
	}

	pub fn contains_all(&self, required_classes: &[ItemClass]) -> bool {
		let mut classes: Vec<ItemClass> = self.iter()
			.map(|x| x.get_class())
			.collect();

		for req_class in required_classes {
			if let Some(pos) = classes
					.iter()
					.position(|x| x == req_class) {

				classes.remove(pos);
			} else { return false; }
		}

		true
	}

	pub fn iter(&self) -> slice::Iter<Item> {
		self.as_ref().iter()
	}

	pub fn as_ref(&self) -> &[Item] {
		self.items.as_ref()
	}

	#[allow(dead_code)]
	pub fn as_mut(&mut self) -> &mut [Item] {
		self.items.as_mut()
	}

	pub fn get_item_vec(&mut self) -> &mut Vec<Item> {
		&mut self.items
	}

	pub fn get(&self, i: usize) -> &Item {
		&self.items[i]
	}

	pub fn has_index(&self, i: usize) -> bool {
		i < self.items.len()
	}

	pub fn reduce(&mut self, items: &[ItemClass]) {
		for &item in items {
			let p = self.items
				.iter()
				.position(|x| x.get_class() == item)
				.unwrap();
			self.items.remove(p);
		}
	}

	pub fn get_weight(&self) -> u32 {
		self.iter()
			.map(|x| (*x).get_class()
						  .get_weight()
			)
			.sum()
	}

	pub fn damage(&mut self, damage: Damage) {
		let mut items = Vec::new();
		mem::swap(&mut items, &mut self.items);
		
		for mut item in items.into_iter() {
			if !item.damage(damage) {
				self.items.push(item);
			}
		}
	}
}

pub fn melee_aim(mouse: Vec2f) -> Vec<Vec2i> {
	let vec = vec![(0,1), (0,-1), (1,0), (-1,0)];
	let tile_center = |v: Vec2i| v.to_f() + 0.5;
	let f = |&w: &Vec2i| ((tile_center(w) - mouse).magnitude_sqr() * 1000.0) as i32;
	let ret = vec![vec.into_iter()
		.map(Vec2i::from)
		.min_by_key(f)
		.unwrap()];
	ret
}
