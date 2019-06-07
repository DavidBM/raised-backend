use game::engine::structs::Position;
use game::entities::domain::pj::Pj;
use std::sync::RwLock;
use game::structs::PlayerIntention;
use crate::game::entities::domain::world::{WorldUpdate, WorldHistory, World};
use crate::game::entities::{PjMovement, System};
use std::sync::Arc;

#[derive(Debug)]
pub struct Runner {
	version: u64,
	world: Arc<RwLock<WorldHistory>>,
	player_intention_buffer: Vec<PlayerIntention>,
	systems: Vec<Box<dyn System>>
}

impl <'a> Runner {
	pub fn new() -> Runner {
		let initial_world = World::new();
		let world_history = Arc::new(RwLock::new(WorldHistory::new(initial_world)));

		let mut runner = Runner {
			version: 0u64, 
			world: world_history, 
			player_intention_buffer: Vec::new(),
			systems: Vec::new()
		};

		runner.add_system(Box::new(PjMovement {}));

		runner
	}

	fn add_system(&mut self, system: Box<dyn System>) {
		self.systems.push(system);
	}

	pub fn update(&mut self, _elapsed: u32) -> WorldUpdate {
		let mut updates = WorldUpdate::new();

		self.set_player_intentions_in_world();

		for system in self.systems.iter_mut() {
			let world_history = self.world.clone();
			//In order to do the threads we need to make then to send messages to a channel.
			//Provably we shouldn't spanw the threads from there, but in the "add_system" call.
			let effects = system.execute_tick(&world_history.read().unwrap());

			for effect in effects {
				updates.add_pach(effect);
			}
		}

		self.world.write().unwrap().update(updates.clone());

		updates
	}

	fn set_player_intentions_in_world(&mut self) {
		let world = self.world.write().unwrap();

		let world = world.get_current();

		let mut world = match world {
			Some(world) => world,
			None => return (),
		};

		for intention in &self.player_intention_buffer {
			for player in world.players.iter_mut() {
				if player.id == intention.player_id {
					player.intention = Some(intention.intention.clone());
				}
			}
		}
	}

	pub fn add_player(&mut self, player_id: u64) {
		let world = self.world.write().unwrap();

		let world = world.get_current();

		let mut world = match world {
			Some(world) => world,
			None => return (),
		};

		world.players.push(Pj {id: player_id, position: Position {x: 0.0, y: 0.0, z: 0.0}, intention: None});
	}

	pub fn set_players_intention(&mut self, intentions: Vec<PlayerIntention>) {
		for intention in intentions {
			self.player_intention_buffer.push(intention);
		}
	}
}
