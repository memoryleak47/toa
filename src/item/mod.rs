pub mod food;
pub mod wood;
pub mod club;

use world::aim::Aim;
use std::ops::{Deref, DerefMut};
use std::slice;

pub trait ItemClass: Sync {
	fn get_name(&self) -> &'static str;
	fn get_ref(&self) -> &'static dyn ItemClass;
	fn get_weight(&self) -> u32;
	fn build(&self) -> ItemBox;
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]>;
}

pub trait Item {
	fn get_class(&self) -> &'static dyn ItemClass;
	fn damage(&mut self);
	fn is_dead(&self) -> bool;
	fn clone_box(&self) -> ItemBox;
	fn aim(&self) -> Box<dyn Aim>;
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

	pub fn clear_dead_items(&mut self) {
		use std::mem::swap;

		let mut tmp = Vec::new();

		swap(&mut self.items, &mut tmp);
		self.items = tmp.into_iter()
			.filter(|x| !x.is_dead())
			.collect();
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
