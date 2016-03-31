use game::Map;
use game::Player;
use game::PlayerIntention;
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
		self.process_world(actions);
		self.process_players();
		self.send_player_responses()
	}

	fn process_players_messages(&self, elapsed: u64) -> Vec<PlayerIntention> {
		let mut intentions: Vec<PlayerIntention> = Vec::new();

		for player in &self.players {
			let player_intentions = player.process_messages(elapsed);
			intentions.extend_from_slice(&player_intentions);
		}

		return intentions;
	}

	fn process_world(&self, intentions: Vec<PlayerIntention>) {

	}

	fn send_player_responses(&self) {
		unimplemented!();
	}

	fn process_players(&self) {
		unimplemented!();
	}
}
