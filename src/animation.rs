use crate::*;

#[derive(Clone)]
pub struct Animation {
	pub remaining_ticks: u32,
	pub kind: AnimationKind,
}

#[derive(Clone)]
pub enum AnimationKind {
	Damage,
	Burn,
}

impl Animation {
	pub fn new(kind: AnimationKind) -> Animation {
		Animation { remaining_ticks: 10, kind }
	}
}

impl App {
	pub fn tick_animationmap(&mut self)  {
		for t in Pos::iter_all() {
			if let Some(mut x) = self.animationmap.get(t).cloned() {
				if x.remaining_ticks == 0 {
					self.animationmap.set(t, None);
				} else {
					x.remaining_ticks -= 1;
					self.animationmap.set(t, Some(x));
				}
			}
		}
	}

	pub fn animate(&mut self, c: &Command) {
		match c {
			Command::UnitCommand { command: UnitCommand::BurnBuilding, pos } => {
				self.animationmap.set(*pos, Some(Animation::new(AnimationKind::Burn)));
			}
			Command::UnitCommand { command: UnitCommand::Attack(a, b), pos } => {
				let unit = self.world.unitmap.get(*pos).unwrap();
				// TODO extract this into some function
				let rel_tiles = if let Some(i) = a {
					unit.inventory.iter().nth(*i).unwrap().aim(*b)
				} else {
					melee_aim(*b)
				};
				for t in rel_tiles {
					let p = if let Some(x) = pos.map(|p| p + t) { x } else { continue; };
					self.animationmap.set(p, Some(Animation::new(AnimationKind::Damage)));
				}
			}
			_ => {},
		}
	}
}

impl GameObject for AnimationKind {
	fn get_texture_id(&self) -> TextureId {
		match self {
			AnimationKind::Damage => RawTextureId::DamageAnimation.into(),
			AnimationKind::Burn => RawTextureId::BurnAnimation.into(),
		}
	}
	fn get_relative_pos(&self) -> Vec2f { <_>::from(0.) }
	fn get_size(&self) -> Vec2f { <_>::from(1.) }
}