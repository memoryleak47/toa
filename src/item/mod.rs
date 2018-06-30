pub mod food;
pub mod wood;

use std::slice;

pub trait ItemClass {
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
}

impl PartialEq for ItemClass {
	fn eq(&self, other: &ItemClass) -> bool {
		self as *const ItemClass == other as *const ItemClass
	}
}
impl Eq for ItemClass {}
