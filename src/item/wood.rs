use crate::*;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct WoodClass;

#[derive(Clone, Serialize, Deserialize)]
pub struct Wood;

impl ItemClassTrait for WoodClass {
    type Instance = Wood;

    fn get_name() -> &'static str {
        "Wood"
    }
    fn get_weight() -> u32 {
        2
    }
    fn build() -> Item {
        Item::Wood(Wood)
    }
    fn get_recipe() -> Option<&'static [ItemClass]> {
        None
    }
    fn stateless() -> bool {
        true
    }
}

impl ItemTrait for Wood {
    type Class = WoodClass;

    fn get_class(&self) -> ItemClass {
        ItemClass::Wood
    }
    fn damage(&mut self, _: Damage) -> bool {
        true
    }
    fn get_damage(&self) -> Damage {
        Damage(2)
    }
}
