use game::structs::Effect;
use crate::game::entities::domain::Pj;

#[derive(Debug)]
pub struct WorldHistory {
	worlds: Vec<World>
}

impl WorldHistory {
	pub fn new(initial_world: World) -> WorldHistory {
		WorldHistory{worlds: vec![initial_world]}
	}

	pub fn update(&mut self, update: WorldUpdate) {

		if let Some(mut world) = self.get_current() {

			let new_world = world.update(update);

			self.add(new_world);
		}
	}

	pub fn get_current(&self) -> Option<World> {
		let world = self.worlds.last();
		match world {
			Some(world) => Some(world.clone()),
			None => panic!("No actual world! {:?}", self),
		}
	}

	fn add(&mut self, world: World) {
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

	pub fn update(&mut self, update: WorldUpdate) -> World {
		let mut world = self.clone();
		world.version += 1;

		for _patch in &update.patchs {

		}

		self.path = update;

		return world;
	}

	pub fn apply_to_player(&mut self, _player_id: Pj /*missing callback*/) {
		unimplemented!()
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
