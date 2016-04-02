use net::GameClient as Client;
use net::Message as Actions;
use net;
use game::MapPosition;
use game::map::PlayerEffects;
use std::f32::consts::PI as PIf32;

#[derive(Debug)]
pub struct Player {
	pub id: u64,
	net: Client,
	position: MapPosition
}
#[derive(Debug, Clone)]
pub enum Intention {
	Move {
		player_id: u64,
		x: f32,
		y: f32,
		z: f32,
		direction: f32,
	},
	None
}

impl Player {
	pub fn new(client: Client, id: u64) -> Player {
		Player {net: client, position: MapPosition {x: 0f32, y: 0f32, z: 0f32}, id: id}
	}

	pub fn get_actions(&self) -> Option<Vec<Actions>> {
		self.net.get_messages()
	}

	pub fn process_message(&self, message: Actions, elapsed: u64) -> Intention{
		match message {
			Actions::PlayerMove(message) => self.player_move(message, elapsed),
			Actions::PlayerDisconnected => Intention::None,
			_ => Intention::None,
		}
	}

	pub fn process_messages(&self, elapsed: u64) -> Vec<Intention>{
		let messages = self.get_actions();
		let mut intentions: Vec<Intention> = Vec::new();

		if let Some(messages) = messages {
			for message in messages {
				let player_intention = self.process_message(message, elapsed);
				intentions.push(player_intention);
			}
		}

		intentions
	}

	pub fn player_move(&self, action: net::PlayerMove, elapsed: u64) -> Intention {
		let x = self.position.x + action.direction.cos() * action.velocity * elapsed as f32;
		let y = self.position.y + action.direction.sin() * action.velocity * elapsed as f32;
		let mut direction = f32::atan2(y, x);

		if direction < 0_f32 {
			direction += 2_f32 * PIf32;
		}

		Intention::Move {
			player_id: self.id,
			x: x,
			y: y,
			z: self.position.z,
			direction: direction
		}
	}

	pub fn apply_effect(&mut self, effect: &PlayerEffects) {
		match effect {
			&PlayerEffects::Position{x, y, z, ..} => self.set_position(x, y, z),
		}
	}

	pub fn set_position(&mut self, x:f32, y:f32, z:f32) {
		println!("Position changed, x:{} y: {} z:{}", x, y, z);
		self.position = MapPosition{x: x, y: y, z: z};
	}
}
