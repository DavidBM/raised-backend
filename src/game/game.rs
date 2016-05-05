use game::World;
use game::Player;
use game::PlayerIntention;
use std::thread::sleep;
use std::time::Duration;
use time::precise_time_ns;

#[derive(Debug)]
pub struct Game {
	world: World,
	players: Vec<Player>
}

impl<'a> Game {
	pub fn new(world: World) -> Game {
		Game { world: world, players: Vec::new() }
	}

	pub fn add_player(&mut self, player: Player) {
		self.players.push(player);
	}

	pub fn start(&mut self){
		let frame_duration = 1_000_000_000u32 / 40u32;

		loop {
			let time = precise_time_ns();

			self.compute(frame_duration);

			let duration = (precise_time_ns() - time) as u32;

			if cfg!(debug_assertions){
				print!("Time ns: {:.5} - Sleep time ns: {:.5}     \x0D", duration, frame_duration as i32 - duration as i32);
			}

			sleep(Duration::new(0, frame_duration - duration));
		}
	}

	fn compute(&mut self, elapsed: u32) {
		let world_update = self.world.update(elapsed);
	}

	fn process_players_messages(&self, elapsed: u32) -> Vec<PlayerIntention> {
		let mut intentions: Vec<PlayerIntention> = Vec::new();

		for player in &self.players {
			let player_intentions = player.process_messages(elapsed);
			intentions.extend_from_slice(&player_intentions);
		}

		return intentions;
	}

	fn delete_players(&mut self) {
		self.players.retain(|player| player.is_in_game());
	}

	fn get_player_by_id(&'a mut self, id: u64) -> Option<&'a mut Player> {
		self.players.iter_mut().find(|player| player.id == id)
	}
}
