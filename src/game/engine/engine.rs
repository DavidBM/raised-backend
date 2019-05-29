use crate::game::engine::world::{WorldUpdate, WorldHistory, World, WorldPatch};
use crate::game::structs::Intention;
use crate::game::engine::pj::Pj;
use std::sync::Arc;

#[derive(Debug)]
pub struct Runner {
	pjs: Vec<Pj>,
	version: u64,
	world: Arc<WorldHistory>,
	patch_buffer: Vec<WorldPatch>
}

impl <'a> Runner {
	pub fn new() -> Runner {
		let initial_world = World::new();
		let world_history = Arc::new(WorldHistory::new(initial_world));
		Runner {pjs: Vec::new(), version: 0u64, world: world_history, patch_buffer: Vec::new()}
	}

	pub fn update(&self, _elapsed: u32) -> WorldUpdate {
		let updates = WorldUpdate::new();
		//Get intentions
		//Execute updates

		updates
	}

	pub fn add_player(&mut self, player: Pj) {
		let patch = WorldPatch::NewUser(player);
		self.patch_buffer.push(patch);
	}

	pub fn set_players_intention(&mut self, player_id: u64, intentions: Vec<Intention>) {
		for intention in intentions {
			let patch = WorldPatch::PlayerIntention {id: player_id, intention: intention};
			self.patch_buffer.push(patch);
		}
	}
}
