use game::Map;
use game::Player;
use game::PlayerIntention;
use game::map::PlayerEffects;
use std::thread::sleep;
use std::time::Duration;
use time::precise_time_ns;

#[derive(Debug)]
pub struct Game {
	map: Map,
	players: Vec<Player>
}

impl Game {
	pub fn new(map: Map) -> Game {
		Game { map: map, players: Vec::new() }
	}

	pub fn add_player(&mut self, player: Player) {
		self.players.push(player);
	}

	pub fn start(&mut self){
		let frame_duration = 1_000_000_000u64 / 40u64;

		let mut last_time = 0u64;

		loop {
			let time = precise_time_ns();

			self.compute(time - last_time);

			last_time = precise_time_ns();
			let duration = last_time - time;
			let sleep_time = (frame_duration - duration) as u32;

			if sleep_time > 0 {
				sleep(Duration::new(0, sleep_time));
			} else {
				println!("Server needs more power! ò_ó -> {:?}", duration);
			}
		}
	}

	fn compute(&mut self, elapsed: u64) {
		let actions = self.process_players_messages(elapsed);
		let effects = self.process_world(&actions);
		self.apply_effects_on_players(&effects);
		//self.delete_players();
	}

	fn process_players_messages(&self, elapsed: u64) -> Vec<PlayerIntention> {
		let mut intentions: Vec<PlayerIntention> = Vec::new();

		for player in &self.players {
			let player_intentions = player.process_messages(elapsed);
			intentions.extend_from_slice(&player_intentions);
		}

		return intentions;
	}

	fn process_world(&self, intentions: &Vec<PlayerIntention>) -> Vec<PlayerEffects> {
		let mut player_effects: Vec<PlayerEffects> = Vec::new();

		for intention in intentions {
			let intentions = self.map.process_player_intention(intention);
			if let Some(intentions) = intentions {
				for intention in intentions {
					player_effects.push(intention);
				}
			}
		}

		player_effects
	}

	fn apply_effects_on_players(&mut self, effects: &Vec<PlayerEffects>) {
		//Could be better whith the macro maybe!, but is not implemented
		for effect in effects {
		    let player_id = unwrap_or_return!(effect.get_id(), ());
		    let player_index = unwrap_or_return!(self.players.iter().position(|player| player.id == player_id), ());
		    let player = unwrap_or_return!(self.players.get_mut(player_index), ());
		    player.apply_effect(effect);
		}
	}

	fn delete_players(&self) {
		unimplemented!();
	}
}
