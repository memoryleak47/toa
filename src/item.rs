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
