use game::engine::world_update::*;
use game::structs::Intention;
use game::engine::pj::Pj;

#[derive(Debug)]
pub struct World {
	pjs: Vec<Pj>,
	version: u64
}

impl <'a> World {
	pub fn new() -> World {
		World {pjs: Vec::new(), version: 0u64}
	}

	pub fn add_player(&mut self, player: Pj) {
		self.pjs.push(player);
	}

	pub fn update(&self, elapsed: u32) -> WorldUpdate {
		let updates = WorldUpdate::new(1u64);



		updates
	}

	pub fn set_players_intention(&mut self, player_id: u64, intentions: Vec<Intention>) {
		let pj = self.get_player_by_id(player_id);

		if let Some(pj) = pj {
			for intention in intentions {
				pj.set_players_intention(intention);
			}
		}
	}

	fn get_player_by_id(&'a mut self, id: u64) -> Option<&'a mut Pj> {
		self.pjs.iter_mut().find(|pj| pj.id == id)
	}
}
