pub mod food;
pub mod wood;
pub mod club;

use sfml::system::Vector2u;

use std::ops::{Deref, DerefMut};
use std::slice;
use std::mem;

use crate::world::aim::Aim;
use crate::world::World;
use crate::world::damage::Damage;

pub trait ItemClass: Sync {
	fn get_name(&self) -> &'static str;
	fn get_ref(&self) -> &'static dyn ItemClass;
	fn get_weight(&self) -> u32;
	fn build(&self) -> ItemBox;
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]>;
}

pub trait Item {
	fn get_class(&self) -> &'static dyn ItemClass;
	fn damage(&mut self, damage: Damage) -> bool; // returns whether the item got destroyed
	fn clone_box(&self) -> ItemBox;
	fn aim(&self) -> Box<dyn Aim>;
	fn is_execable(&self, _p: Vector2u, _w: &World) -> bool { false }
	fn exec(&self, _p: Vector2u, _w: &mut World) {}
}

pub struct ItemBox(pub Box<dyn Item>);

pub struct Inventory {
	items: Vec<ItemBox>,
}

impl Inventory {
	pub fn new() -> Inventory {
		Inventory { items: Vec::new() }
	}

	pub fn push(&mut self, item: ItemBox) {
		self.items.push(item);
	}

	pub fn remove(&mut self, index: usize) -> ItemBox {
		self.items.remove(index)
	}

	pub fn contains_all(&self, required_classes: &[&'static dyn ItemClass]) -> bool {
		let mut classes: Vec<&'static dyn ItemClass> = self.iter()
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

	pub fn iter(&self) -> slice::Iter<ItemBox> {
		self.as_ref().iter()
	}

	pub fn as_ref(&self) -> &[ItemBox] {
		self.items.as_ref()
	}

	#[allow(dead_code)]
	pub fn as_mut(&mut self) -> &mut [ItemBox] {
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

	pub fn get_item_vec(&mut self) -> &mut Vec<ItemBox> {
		&mut self.items
	}

	pub fn reduce(&mut self, items: &[&'static dyn ItemClass]) {
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
			.map(|x| (**x).get_class()
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
		let items = self.items.iter()
			.map(|x| x.clone())
			.collect();
		Inventory { items }
	}
}

impl PartialEq for dyn ItemClass {
	fn eq(&self, other: &dyn ItemClass) -> bool {
		self as *const dyn ItemClass == other as *const dyn ItemClass
	}
}

impl Eq for dyn ItemClass {}

impl Clone for ItemBox {
	fn clone(&self) -> ItemBox {
		self.clone_box()
	}
}

impl Deref for ItemBox {
	type Target = dyn Item;

	fn deref(&self) -> &Self::Target {
		self.0.as_ref()
	}
}

impl DerefMut for ItemBox {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0.as_mut()
	}
}
