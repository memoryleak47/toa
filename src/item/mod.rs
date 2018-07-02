pub mod food;
pub mod wood;

use std::slice;

pub trait ItemClass: Sync {
	fn get_name(&self) -> &'static str;
	fn get_ref(&self) -> &'static ItemClass;
	fn get_mass(&self) -> u32;
	fn build(&self) -> Box<Item>;
}

pub trait Item {
	fn get_class(&self) -> &'static ItemClass;
	fn damage(&mut self);
	fn is_dead(&self) -> bool;
}

pub struct Inventory {
	items: Vec<Box<Item>>,
}

impl Inventory {
	pub fn new() -> Inventory {
		Inventory { items: Vec::new() }
	}

	pub fn push(&mut self, item: Box<Item>) {
		self.items.push(item);
	}

	pub fn contains_all(&self, required_classes: &[&'static ItemClass]) -> bool {
		let mut classes: Vec<&'static ItemClass> = self.iter()
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

	pub fn iter(&self) -> slice::Iter<Box<Item>> {
		self.as_ref().iter()
	}

	pub fn as_ref(&self) -> &[Box<Item>] {
		self.items.as_ref()
	}

	pub fn as_mut(&mut self) -> &mut [Box<Item>] {
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

	pub fn get_item_vec(&mut self) -> &mut Vec<Box<Item>> {
		&mut self.items
	}

	pub fn reduce(&mut self, items: &[&'static ItemClass]) {
		for &item in items {
			let p = self.items
				.iter()
				.position(|x| x.get_class() == item)
				.unwrap();
			self.items.remove(p);
		}
	}
}

impl PartialEq for ItemClass {
	fn eq(&self, other: &ItemClass) -> bool {
		self as *const ItemClass == other as *const ItemClass
	}
}
impl Eq for ItemClass {}
