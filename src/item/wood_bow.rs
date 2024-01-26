use crate::*;

lazy_static! {
    static ref RECIPE: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Wood];
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct WoodBowClass;

#[derive(Clone, Serialize, Deserialize)]
pub struct WoodBow;

impl ItemClassTrait for WoodBowClass {
    type Instance = WoodBow;

    fn get_name() -> &'static str {
        "WoodBow"
    }
    fn get_weight() -> u32 {
        8
    }
    fn build() -> Item {
        Item::WoodBow(WoodBow)
    }
    fn get_recipe() -> Option<&'static [ItemClass]> {
        Some(&RECIPE[..])
    }
    fn stateless() -> bool {
        false
    }
}

impl ItemTrait for WoodBow {
    type Class = WoodBowClass;

    fn get_class(&self) -> ItemClass {
        ItemClass::WoodBow
    }
    fn damage(&mut self, _: Damage) -> bool {
        true
    }
    fn get_damage(&self) -> Damage {
        Damage(3)
    }
    fn aim(&self, v: Vec2f) -> Vec<Vec2i> {
        iter::once(v.to_i())
            .filter(|p| p.magnitude_sqr() <= 3 * 3)
            .collect()
    }
}
