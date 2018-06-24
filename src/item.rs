#[derive(Debug, Copy, Clone, PartialEq)]
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

	pub fn contains_all(&self, kinds: &[ItemKind]) -> bool {
		let mut items = self.items.clone();
		for kind in kinds {
			if let Some(pos) = items
					.iter()
					.position(|x| x.kind == *kind) {

				items.remove(pos);
			} else { return false; }
		}

		true
	}

	pub fn as_ref(&self) -> &[Item] {
		self.items.as_ref()
	}

	pub fn as_mut(&mut self) -> &mut [Item] {
		self.items.as_mut()
	}
}
