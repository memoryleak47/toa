use crate::*;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct IronClass;

#[derive(Clone, Serialize, Deserialize)]
pub struct Iron;

impl ItemClassTrait for IronClass {
    type Instance = Iron;

    fn get_name() -> &'static str {
        "Iron"
    }
    fn get_weight() -> u32 {
        3
    }
    fn build() -> Item {
        Item::Iron(Iron)
    }
    fn get_recipe() -> Option<&'static [ItemClass]> {
        None
    }
    fn stateless() -> bool {
        true
    }
}

impl ItemTrait for Iron {
    type Class = IronClass;

    fn get_class(&self) -> ItemClass {
        ItemClass::Iron
    }
    fn damage(&mut self, _: Damage) -> bool {
        true
    }
    fn get_damage(&self) -> Damage {
        Damage(3)
    }
}
