mod food;
mod wood;
mod wood_sword;
mod stone;
mod iron;
mod iron_sword;
mod wood_bow;
mod settlement_kit;
mod long_sword;
mod lance;

use std::slice;
use std::mem;

use crate::vec::Pos;
use crate::aim::Aim;
use crate::world::World;
use crate::damage::Damage;

use self::food::Food;
use self::wood::Wood;
use self::wood_sword::WoodSword;
use self::stone::Stone;
use self::iron::Iron;
use self::iron_sword::IronSword;
use self::wood_bow::WoodBow;
use self::settlement_kit::SettlementKit;
use self::long_sword::LongSword;
use self::lance::Lance;

trait ItemTrait {
	type Class: ItemClassTrait + Sized;

	fn get_class(&self) -> ItemClass;

	// returns whether the item got destroyed
	fn damage(&mut self, damage: Damage) -> bool;
	fn aim(&self) -> Aim;
	fn is_execable(&self, _p: Pos, _w: &World) -> bool { false }
	fn exec(&self, _p: Pos, _w: &mut World) { panic!("default ItemTrait::exec() was called!"); }
}

trait ItemClassTrait {
	type Instance: ItemTrait + Sized;

	fn get_name() -> &'static str;
	fn get_weight() -> u32;
	fn build() -> Item;
	fn get_recipe() -> Option<&'static [ItemClass]>;
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
			pub fn damage(&mut self, damage: Damage) -> bool			{ match self { $( Item::$x(a) => a.damage(damage) ),* } }
			pub fn aim(&self) -> Aim									{ match self { $( Item::$x(a) => a.aim() ),* } }
			pub fn is_execable(&self, p: Pos, w: &World) -> bool		{ match self { $( Item::$x(a) => a.is_execable(p, w) ),* } }
			pub fn exec(&self, p: Pos, w: &mut World)					{ match self { $( Item::$x(a) => a.exec(p, w) ),* } }
		}

		impl ItemClass {
			pub fn get_name(&self) -> &'static str						{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::get_name() ),* } }
			pub fn get_weight(&self) -> u32								{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::get_weight() ),* } }
			pub fn build(&self) -> Item									{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::build() ),* } }
			pub fn get_recipe(&self) -> Option<&'static [ItemClass]> 	{ match self { $( ItemClass::$x => <$x as ItemTrait>::Class::get_recipe() ),* } }
		}
	};
}

setup!(Food, Wood, WoodSword, Stone, Iron, IronSword, WoodBow, SettlementKit, LongSword, Lance);

#[derive(Serialize, Deserialize)]
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

	pub fn get_info_string(&self) -> String {
		let mut s = String::new();
		s.push('[');
		let tmp: Vec<&'static str> = self.iter()
			.map(|x| x.get_class().get_name())
			.collect();
		s.push_str(&(&tmp[..]).join(", "));
		s.push(']');
		s
	}

	pub fn get_item_vec(&mut self) -> &mut Vec<Item> {
		&mut self.items
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

impl Clone for Inventory {
	fn clone(&self) -> Inventory {
		Inventory { items: self.items.clone() }
	}
}
