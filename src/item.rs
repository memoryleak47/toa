#[derive(Debug)]
pub enum ItemKind {
	Food,
	Wood,
	Stone,
	Iron,
}

#[derive(Debug)]
pub struct Item {
	health: u32,
	kind: ItemKind,
}
