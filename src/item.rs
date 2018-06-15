#[derive(Debug, Copy, Clone)]
pub enum ItemKind {
	Food,
	Wood,
	Stone,
	Iron,
}

#[derive(Debug, Copy, Clone)]
pub struct Item {
	pub health: u32,
	pub kind: ItemKind,
}

#[derive(Clone, Debug)]
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

	pub fn iter(&self) -> &[Item] {
		&self.items[..]
	}

	pub fn iter_mut(&mut self) -> &mut [Item] {
		&mut self.items[..]
	}
}
