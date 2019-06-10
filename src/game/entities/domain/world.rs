use crate::game::structs::Effect;
use crate::game::entities::domain::Pj;

#[derive(Debug)]
pub struct WorldHistory {
	worlds: Vec<World>
}

impl WorldHistory {
	pub fn new(initial_world: World) -> WorldHistory {
		WorldHistory{worlds: vec![initial_world]}
	}

	pub fn get_current(&self) -> World {
		let world = self.worlds.last();
		match world {
			Some(world) => world.clone(),
			None => panic!("No actual world! {:?}", self),
		}
	}

	pub fn update(&mut self, world: World) {
		self.worlds.push(world);
	}
}

#[derive(Debug, Clone)]
pub struct World {
	pub players: Vec<Pj>,
	pub version: u64,
	pub path: WorldUpdate,
}

impl World {
	pub fn new() -> World {
		World {players: Vec::new(), version: 0u64, path: WorldUpdate::new()}
	}

	pub fn apply_to_player(&mut self, player_id: &u64, callback: impl Fn(&mut Pj) -> ()) {
		let player = self.players.iter_mut().find(|pj| { pj.id == *player_id });

		match player {
			Some(player) => callback(player),
			None => (),
		}
	}
}

#[derive(Debug, Clone)]
pub struct WorldUpdate {
	pub patchs: Vec<Effect>,
	pub time: u64
}

impl WorldUpdate {
	pub fn new () -> WorldUpdate {
		WorldUpdate {patchs: Vec::new(), time: 0u64}
	}

	pub fn add_pach(& mut self, patch: Effect) {
		self.patchs.push(patch);
	}
}
