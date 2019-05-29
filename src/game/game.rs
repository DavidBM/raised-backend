use crate::game::Engine;
use crate::game::Player;
use std::thread::sleep;
use std::time::Duration;
use time::precise_time_ns;
use crate::game::engine::pj::Pj;
use crate::config::engine::TICK_TIME;

#[derive(Debug)]
pub struct Game {
	runner: Engine,
	players: Vec<Player>
}

impl<'a> Game {
	pub fn new() -> Game {
		Game { runner: Engine::new(), players: Vec::new() }
	}

	pub fn add_player(&mut self, player: Player) {
		self.runner.add_player(Pj::new(player.id));
		self.players.push(player);
	}

	pub fn start(&mut self){
		loop {
			let time = precise_time_ns();

			self.compute(TICK_TIME);

			let duration = (precise_time_ns() - time) as u32;

			if cfg!(debug_assertions){
				print!("Tick time ns: {:.5} - Sleep time ns: {:.5}     \x0D", duration, TICK_TIME as i32 - duration as i32);
			}

			if duration < TICK_TIME {
				sleep(Duration::new(0, TICK_TIME - duration));
			}
		}
	}

	fn compute(&mut self, elapsed: u32) {
		self.update_players_updates();
		self.runner.update(elapsed);
	}

	fn update_players_updates(&mut self){

		for player in &self.players {
			let updates = player.get_updates();
			self.runner.set_players_intention(player.id, updates);
		}
	}

	fn get_player_by_id(&'a mut self, id: u64) -> Option<&'a mut Player> {
		self.players.iter_mut().find(|player| player.id == id)
	}
}
