use crate::*;

const C: usize = (MAP_SIZE_X as usize) * (MAP_SIZE_Y as usize);

#[derive(Clone, Serialize, Deserialize)]
pub struct TileMap<T: Clone>(Vec<T>);

impl<T: Clone> TileMap<T> {
	pub fn new(t: T) -> TileMap<T> {
        let tilemap: Vec<T> = iter::repeat(t).take(C).collect();
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
