use crate::game::Engine;
use crate::game::Player;
use std::thread::sleep;
use std::time::Duration;
use time::precise_time_ns;
use crate::config::engine::TICK_TIME;
use crate::game::structs::Effect;

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
		self.runner.add_player(player.id);
		self.players.push(player);
	}

	pub fn start(&mut self){
		loop {
			let time = precise_time_ns();

			self.compute(TICK_TIME);

			let duration = (precise_time_ns() - time) as u32;

			info!("Tick time ms: {:.10} - Sleep time ms: {:.10}", 
				duration as f64 / 1000000.0, 
				TICK_TIME as f64 / 1000000.0 as f64 - duration as f64 / 1000000.0 as f64
			);

			if duration < TICK_TIME {
				sleep(Duration::new(0, TICK_TIME - duration));
			}
		}
	}

	fn compute(&mut self, elapsed: u32) {
		self.update_players_updates();
		let updates = self.runner.update(elapsed);

		trace!("World updated {:?}", updates);

		for update in updates.patchs {
			match update {
				Effect::PlayerMoved {player_id, ..}=> {
					let player = self.get_player_by_id(player_id);
						match player {
							Some(player) => player.send(&update),
							None => (),
						}
					()
				},
				_ => ()
			}
		}
	}

	fn update_players_updates(&mut self){

		for player in &self.players {
			let updates = player.get_updates();
			trace!("Player updates: {:?}", updates);
			self.runner.set_players_intention(updates);
		}
	}

	fn get_player_by_id(&'a mut self, id: u64) -> Option<&'a mut Player> {
		self.players.iter_mut().find(|player| player.id == id)
	}
}
