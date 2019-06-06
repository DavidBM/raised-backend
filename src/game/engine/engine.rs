use std::sync::RwLock;
use std::thread;
use game::structs::PlayerIntention;
use crate::game::entities::domain::world::{WorldUpdate, WorldHistory, World};
use crate::game::structs::Intention;

use crate::game::entities::{PjMovement, System};
use std::sync::Arc;

#[derive(Debug)]
pub struct Runner {
	version: u64,
	world: Arc<RwLock<WorldHistory>>,
	intention_buffer: Vec<PlayerIntention>,
	systems: Vec<Box<dyn System>>
}

impl <'a> Runner {
	pub fn new() -> Runner {
		let initial_world = World::new();
		let world_history = Arc::new(RwLock::new(WorldHistory::new(initial_world)));

		let mut runner = Runner {
			version: 0u64, 
			world: world_history, 
			intention_buffer: Vec::new(),
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

		for system in self.systems.iter_mut() {
			let world_history = self.world.clone();
			//In order to do the threads we need to make then to send messages to a channel.
			//Provably we shouldn't spanw the threads from there, but in the "add_system" call.
			let intentions_effects = system.execute_tick(&world_history.read().unwrap(), self.intention_buffer.clone());
			updates.add_pach(intentions_effects);		
		}

		let mut world = self.world.write().unwrap();

		world.update(updates.clone());

		updates
	}

	pub fn add_player(&mut self, player_id: u64) {
		self.intention_buffer.push(PlayerIntention {intention: Intention::ConnectPlayer, player_id: player_id});
	}

	pub fn set_players_intention(&mut self, intentions: Vec<PlayerIntention>) {
		for intention in intentions {
			self.intention_buffer.push(intention);
		}
	}
}
