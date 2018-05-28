use std::mem;
use std::ptr;

use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::system::{Vector2f, Vector2u};

use view::View;
use misc::vector_uf;

use world::{World, TILESIZE, TILESIZE_VEC, MAP_SIZE_X, MAP_SIZE_Y};
use world::terrainmap::Terrain;
use item::ItemKind;

lazy_static! {
static ref BUILDING_PLANS: [BuildingPlan<'static>; 1] = [
		BuildingPlan {
			building: Box::new(Building {
				kind: BuildingKind::Farm { stamina: 0 },
				health: 100
			}),
			required_stamina: 100,
			required_terrain: Some(Terrain::GRASS),
			required_resources: &[],
		},
	];
}

#[derive(Debug)]
pub struct BuildingPlan<'a> {
	building: Box<Building>,
	required_stamina: u32,
	required_terrain: Option<Terrain>,
	required_resources: &'a [(ItemKind, u32)],
}

#[derive(Debug, Clone)]
pub enum BuildingKind {
	InConstruction { required_stamina: u32, building: Box<Building> },
	Spawn { owner: u32, food: u32 },
	Farm { stamina: u32 },
	Sawmill { stamina: u32 },
	StoneMine { stamina: u32 },
	IronMine { stamina: u32 },
}

#[derive(Debug, Clone)]
pub struct Building {
	kind: BuildingKind,
	health: u32
}

impl BuildingKind {
	fn get_color(&self) -> Color {
		match self {
			BuildingKind::Spawn { owner, .. } =>  {
				if *owner == 0 {
					Color::rgba(255, 0, 0, 100)
				} else {
					Color::rgba(0, 0, 255, 100)
				}
			},
			BuildingKind::InConstruction { .. } => Color::rgb(100, 100, 100),
			_ => Color::rgb(50, 50, 50) // TODO
		}
	}
}

pub fn new_buildingmap() -> [[Option<Building>; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut buildingmap: [[Option<Building>; MAP_SIZE_Y]; MAP_SIZE_X];
	unsafe {
		buildingmap = mem::uninitialized();
	}

	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			unsafe {
				ptr::write(&mut buildingmap[x][y], None);
			}
		}
	}

	buildingmap[MAP_SIZE_X / 2][0] = Some(Building { health: 100, kind: BuildingKind::Spawn { owner: 0, food: 0 }});
	buildingmap[MAP_SIZE_X / 2][MAP_SIZE_Y - 1] = Some(Building { health: 100, kind: BuildingKind::Spawn { owner: 1, food: 0 }});

	buildingmap
}

impl World {
	pub fn render_buildingmap(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(building) = self.buildingmap[x][y].as_ref() {
					let posf = Vector2f::new(x as f32, y as f32);

					let mut shape = RectangleShape::new();
					shape.set_fill_color(&building.kind.get_color());
					shape.set_position((posf - view.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
					shape.set_size(TILESIZE_VEC());
					window.draw(&shape);
				}
			}
		}
	}

	pub fn get_building(&self, p: Vector2u) -> Option<&Building> {
		self.buildingmap[p.x as usize][p.y as usize].as_ref()
	}
}
