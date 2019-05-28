use game::engine::Pj;
use game::structs::Intention;
use config::engine::TICK_TIME;

#[derive(Debug)]
pub struct WorldHistory {
	worlds: Vec<World>
}

impl WorldHistory {
	pub fn new(initial_world: World) -> WorldHistory {
		WorldHistory{worlds: vec![initial_world]}
	}

	fn get_at(&self, past_time: u32) -> Option<World> {
		let iterations = past_time / TICK_TIME;
		let index = self.worlds.len() as u32 - iterations;
		let world = self.worlds.get(index as usize);

		match world {
			Some(world) => Some(world.clone()),
			None => None,
		}
	}

	pub fn update(&mut self, update: WorldUpdate) {

		if let Some(mut world) = self.get_actual() {

			let new_world = world.update(update);

			self.add(new_world);
		}
	}

	fn get_actual(&self) -> Option<World> {
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
	players: Vec<Pj>,
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

		for patch in &update.patchs {

		}

		self.path = update;

		return world;
	}

	fn get_version(version: u64) -> World {
		unimplemented!();
	}
}
#[derive(Debug, Clone)]
pub struct WorldUpdate {
	pub patchs: Vec<WorldPatch>,
	pub time: u64
}

impl WorldUpdate {
	pub fn new () -> WorldUpdate {
		WorldUpdate {patchs: Vec::new(), time: 0u64}
	}

	pub fn add_pach(& mut self, patch: WorldPatch) {
		self.patchs.push(patch);
	}
}


#[derive(Debug, Clone)]
pub enum WorldPatch {
    NewUser (Pj),
    PlayerIntention {id: u64, intention: Intention}
}