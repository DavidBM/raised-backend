use net::GameClient as Client;
use net::ClientPacket;
use net::SendMessage;
use net::packets as actions;
use game::MapPosition;
use game::structs::PlayerEffect;
use game::structs::Intention;

#[derive(Debug)]
pub struct Player {
	pub id: u64,
	net: Client,
	in_game: bool,
}

impl Player {
	pub fn new(client: Client, id: u64) -> Player {
		Player {net: client, id: id, in_game: true}
	}

	pub fn get_actions(&self) -> Option<Vec<ClientPacket>> {
		self.net.get_messages()
	}

	pub fn process_message(&self, message: ClientPacket, elapsed: u32) -> Intention{
		match message {
			ClientPacket::Move(message) => self.player_move(message, elapsed),
			ClientPacket::Disconnected => Intention::None,
			_ => Intention::None,
		}
	}

	pub fn process_messages(&self, elapsed: u32) -> Vec<Intention>{
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

	pub fn player_move(&self, action: actions::Move, elapsed: u32) -> Intention {
		Intention::Move {
			player_id: self.id,
			direction: action.direction
		}
	}

	pub fn is_in_game(&self) -> bool {
		self.in_game
	}

	pub fn get_client_id(&self) -> String {
		self.net.get_id()
	}

	pub fn send(&self, message: &SendMessage) {
		self.net.send(message);
	}
}
