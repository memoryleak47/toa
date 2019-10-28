use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::{Deserializer};

use crate::config::{MAP_SIZE_X, MAP_SIZE_Y};
use crate::vec::Pos;

const C: usize = (MAP_SIZE_X as usize) * (MAP_SIZE_Y as usize);

#[derive(Clone)]
pub struct TileMap<T: Clone>([T; C]);

impl<T: Clone> TileMap<T> {
	pub fn new(t: T) -> TileMap<T> {
		use std::{mem, ptr};

        let mut tilemap: [T; C] = unsafe { mem::uninitialized() };

        for tile in &mut tilemap[..] {
            unsafe { ptr::write(tile, t.clone()); }
		}

		TileMap(tilemap)
	}

	pub fn get(&self, p: Pos) -> &T {
		let idx = pos_to_index(p);
		&self.0[idx]
	}

	pub fn get_mut(&mut self, p: Pos) -> &mut T {
		let idx = pos_to_index(p);
		&mut self.0[idx]
	}

	pub fn set(&mut self, p: Pos, t: T) {
		let idx = pos_to_index(p);
		self.0[idx] = t;
	}
}

fn pos_to_index(p: Pos) -> usize {
	((*p).x as usize) + ((*p).y as usize) * (MAP_SIZE_X as usize)
}

// TODO: fix hacky serialization
impl<T: Clone + Serialize> Serialize for TileMap<T> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let v: Vec<T> = (&self.0[..]).iter()
			.cloned()
			.collect();

		v.serialize(serializer)
	}
}

impl<'de, T: Clone + Deserialize<'de>> Deserialize<'de> for TileMap<T> {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		<Vec<T>>::deserialize(deserializer)
			.map(|v| {
				use std::{mem, ptr};

				let mut tilemap: [T; C] = unsafe { mem::uninitialized() };

				for (i, tile) in tilemap.iter_mut().enumerate() {
					unsafe { ptr::write(tile, v[i].clone()); }
				}

				TileMap(tilemap)
			})
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptTileMap<T: Clone>(TileMap<Option<T>>);

impl<T: Clone> OptTileMap<T> {
	pub fn new() -> OptTileMap<T> {
		OptTileMap(TileMap::new(None))
	}

	pub fn get(&self, p: Pos) -> Option<&T> {
		self.0.get(p).as_ref()
	}

	pub fn get_raw(&self, p: Pos) -> &Option<T> {
		self.0.get(p)
	}

	pub fn get_mut(&mut self, p: Pos) -> Option<&mut T> {
		self.0.get_mut(p).as_mut()
	}

	pub fn get_mut_raw(&mut self, p: Pos) -> &mut Option<T> {
		self.0.get_mut(p)
	}

	pub fn set(&mut self, p: Pos, t: Option<T>) {
		self.0.set(p, t);
	}
}
