use game::Map;
use game::Player;
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

			let after_compute_time = precise_time_ns();

			let duration = (after_compute_time - time) / 1000u64;

			let sleep_time = (frame_duration - duration) as u32;

			let sleep_time = Duration::new(0, sleep_time);

			last_time = after_compute_time;

			sleep(sleep_time);
		}
	}

	fn compute(&mut self, elapsed: u64) {
		for player in &self.players {
			let messages = player.get_actions();

			if let Some(messages) = messages {
				for message in messages {
					player.process_message(message, elapsed);
				}
			}
		}
	}
}
