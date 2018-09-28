use toalib::world::aim::Aim;

#[derive(Debug, Copy, Clone)]
pub enum ItemUnitMode {
	Drop, Take, ChangeMainItem, Exec
}

pub enum UnitMode {
	Normal,
	Attack { aim: Aim },
	Build,
	Item { iu_mode: ItemUnitMode, index: usize },
	Craft { index: usize },
}
