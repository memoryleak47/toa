pub mod food;
pub mod wood;
pub mod club;

use objekt;

use std::slice;

pub trait ItemClass: Sync {
	fn get_name(&self) -> &'static str;
	fn get_ref(&self) -> &'static dyn ItemClass;
	fn get_mass(&self) -> u32;
	fn build(&self) -> Box<dyn Item>;
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]>;
}

pub trait Item: objekt::Clone {
	fn get_class(&self) -> &'static dyn ItemClass;
	fn damage(&mut self);
	fn is_dead(&self) -> bool;
}

pub struct Inventory {
	items: Vec<Box<dyn Item>>,
}

impl Inventory {
	pub fn new() -> Inventory {
		Inventory { items: Vec::new() }
	}

	pub fn push(&mut self, item: Box<dyn Item>) {
		self.items.push(item);
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

	pub fn iter(&self) -> slice::Iter<Box<dyn Item>> {
		self.as_ref().iter()
	}

	pub fn as_ref(&self) -> &[Box<dyn Item>] {
		self.items.as_ref()
	}

	pub fn as_mut(&mut self) -> &mut [Box<dyn Item>] {
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

	pub fn get_item_vec(&mut self) -> &mut Vec<Box<dyn Item>> {
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
}

impl Clone for Inventory {
	fn clone(&self) -> Inventory {
		let items = self.items.iter()
			.map(|x| objekt::clone_box(x.as_ref()))
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
