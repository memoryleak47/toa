use crate::*;

#[allow(dead_code)]
pub fn vec2f_to_sfml(v: Vec2f) -> Vector2f {
    Vector2f::new(v.x, v.y)
}

#[allow(dead_code)]
pub fn pos_to_sfml(v: Pos) -> Vector2u {
    Vector2u::new(v.x as u32, v.y as u32)
}

#[allow(dead_code)]
pub fn vector2f_to_toa(v: Vector2f) -> Vec2f {
    Vec2f::new(v.x, v.y)
}

#[allow(dead_code)]
pub fn pos_to_toa(v: Vector2u) -> Pos {
    Pos::build(v.x as i32, v.y as i32).unwrap()
}
