use game::World;
use game::Player;
use game::PlayerIntention;
use game::PlayerNotification;
use game::structs::PlayerEffect;
use std::thread::sleep;
use std::time::Duration;
use time::precise_time_ns;
use net::SendMessage;

#[derive(Debug)]
pub struct Game {
	map: World,
	players: Vec<Player>
}

impl<'a> Game {
	pub fn new(map: World) -> Game {
		Game { map: map, players: Vec::new() }
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
		let actions = self.process_players_messages(elapsed);
		let (effects, notifications) = self.process_world(&actions);
		self.sent_to_players(notifications);
		self.delete_players();
	}

	fn process_players_messages(&self, elapsed: u32) -> Vec<PlayerIntention> {
		let mut intentions: Vec<PlayerIntention> = Vec::new();

		for player in &self.players {
			let player_intentions = player.process_messages(elapsed);
			intentions.extend_from_slice(&player_intentions);
		}

		return intentions;
	}

	fn process_world(&self, intentions: &Vec<PlayerIntention>) -> (Vec<PlayerEffect>, Vec<PlayerNotification>) {
		let mut player_effects: Vec<PlayerEffect> = Vec::new();
		let mut player_notifications: Vec<PlayerNotification> = Vec::new();

		for intention in intentions {
			let effecs = self.map.process_player_intention(intention);

			if let Some(player_effects_returned) = effecs.1 {
				for effect in player_effects_returned {
					player_effects.push(effect);
				}
			}

			if let Some(visible_actions) = effecs.0 {
				for notification in visible_actions {
					player_notifications.push(notification);
				}
			}
		}

		(player_effects, player_notifications)
	}

	fn sent_to_players(&mut self, notifications: Vec<PlayerNotification>) {
		let mut notifications = notifications;
		while let Some(notification) = notifications.pop() {
			self.send_notificatin_to_player(notification);
		}
	}

	fn delete_players(&mut self) {
		self.players.retain(|player| player.is_in_game());
	}

	fn send_notificatin_to_player(&self, notification: PlayerNotification) {
		match notification {
			PlayerNotification::Position{player_id, position, direction, ..} => {
				let client_id = unwrap_or_return!(self.get_client_id(player_id), ());
				let message = SendMessage::PlayerMove{
					x: position.x,
					y: position.y,
					z: position.z,
					direction: direction,
					id: client_id
				};
				for player in &self.players {
					player.send(&message);
				}
			}
		}
	}

	fn get_player_by_id(&'a mut self, id: u64) -> Option<&'a mut Player> {
		self.players.iter_mut().find(|player| player.id == id)
	}

	pub fn get_client_id(&self, id: u64) -> Option<String> {
		for player in &self.players {
			if player.id == id {
				return Some(player.get_client_id());
			}
		}

		None::<String>
	}
}
