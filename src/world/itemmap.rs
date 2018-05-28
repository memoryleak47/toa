use item::Item;
use world::{World, MAP_SIZE_X, MAP_SIZE_Y};

impl World {
}

pub fn new_itemmap() -> [[Option<Item>; MAP_SIZE_Y]; MAP_SIZE_X] {
	[[None; MAP_SIZE_Y]; MAP_SIZE_X]
}
