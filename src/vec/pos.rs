use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Deref;

use crate::*;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pos(Vec2i);

impl Deref for Pos {
    type Target = Vec2i;

    fn deref(&self) -> &Vec2i {
        &self.0
    }
}

impl Pos {
    pub fn map<F: FnOnce(Vec2i) -> Vec2i>(&self, f: F) -> Option<Pos> {
        f(self.0).to_pos()
    }

    pub fn build(x: i32, y: i32) -> Option<Pos> {
        Vec2i::new(x, y).to_pos()
    }

    pub fn next_repeat(&self) -> Pos {
        // contains useless checks
        if let Some(p) = self.map(|a| Vec2i::new(a.x + 1, a.y)) {
            return p;
        }
        if let Some(p) = self.map(|a| Vec2i::new(0, a.y + 1)) {
            return p;
        }
        Pos::build(0, 0).unwrap()
    }

    pub fn next(&self) -> Option<Pos> {
        // contains useless checks
        if let Some(p) = self.map(|a| Vec2i::new(a.x + 1, a.y)) {
            return Some(p);
        }
        if let Some(p) = self.map(|a| Vec2i::new(0, a.y + 1)) {
            return Some(p);
        }
        None
    }

    pub fn iter_all() -> impl Iterator<Item = Pos> {
        PosIter::new()
    }
}

#[allow(dead_code)]
impl Vec2i {
    pub fn round(mut self) -> Pos {
        if self.x < 0 {
            self.x = 0;
        }
        if self.x >= MAP_SIZE_X as i32 {
            self.x = MAP_SIZE_X as i32 - 1;
        }
        if self.y < 0 {
            self.y = 0;
        }
        if self.y >= MAP_SIZE_Y as i32 {
            self.y = MAP_SIZE_Y as i32 - 1;
        }
        self.to_pos().unwrap() // one could remove this check
    }

    pub fn to_pos(self) -> Option<Pos> {
        if self.x < 0 || self.x >= MAP_SIZE_X as i32 {
            return None;
        }
        if self.y < 0 || self.y >= MAP_SIZE_Y as i32 {
            return None;
        }
        Some(Pos(self))
    }
}

impl Display for Pos {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let s = format!("Pos({}, {})", self.x, self.y);
        fmt.write_str(&*s)
    }
}

impl Debug for Pos {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let s = format!("Pos({:?}, {:?})", self.x, self.y);
        fmt.write_str(&*s)
    }
}

struct PosIter(Option<Pos>);

impl PosIter {
    fn new() -> PosIter {
        PosIter(Some(Pos::build(0, 0).unwrap()))
    }
}

impl Iterator for PosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let ret = self.0;
        self.0 = self.0.and_then(|x| x.next());
        ret
    }
}
