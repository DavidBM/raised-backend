use game::World;
use game::Player;
use std::thread::sleep;
use std::time::Duration;
use time::precise_time_ns;
use game::engine::pj::Pj;

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
		self.world.add_player(Pj::new(player.id));
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
		self.update_players_updates();
		self.world.update(elapsed);
	}

	fn update_players_updates(&mut self){

		for player in &self.players {
			let updates = player.get_updates();
			self.world.set_players_intention(player.id, updates);
		}
	}

	fn get_player_by_id(&'a mut self, id: u64) -> Option<&'a mut Player> {
		self.players.iter_mut().find(|player| player.id == id)
	}
}
