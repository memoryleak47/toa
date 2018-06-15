#[derive(Debug, Copy, Clone)]
pub enum ItemKind {
	Food,
	Wood,
	Stone,
	Iron,
}

#[derive(Debug, Copy, Clone)]
pub struct Item {
	health: u32,
	kind: ItemKind,
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

	pub fn iter(&self, item: Item) -> &[Item] {
		&self.items[..]
	}

	pub fn iter_mut(&mut self, item: Item) -> &mut [Item] {
		&mut self.items[..]
	}
}
