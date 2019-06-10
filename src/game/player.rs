use crate::game::structs::PlayerIntention;
use uuid::Uuid;
use crate::net::GameClient as Client;
use crate::net::ClientPacket;
use crate::net::ServerMessage;
use crate::net::packets as actions;
use crate::game::structs::Intention;

#[derive(Debug)]
pub struct Player {
	pub id: u64,
	net: Client,
}

impl Player {
	pub fn new(client: Client, id: u64) -> Player {
		Player {net: client, id: id}
	}

	fn get_update(&self, message: ClientPacket) -> PlayerIntention{
		match message {
			ClientPacket::Move(message) => self.player_move(message),
			ClientPacket::Disconnected => PlayerIntention { player_id: self.id, intention: Intention::DisconnectPlayer},
			_ => PlayerIntention { player_id: self.id, intention: Intention::None},
		}
	}

	pub fn get_updates(&self) -> Vec<PlayerIntention>{
		let messages = self.net.get_messages();
		let mut intentions: Vec<PlayerIntention> = Vec::new();

		if let Some(messages) = messages {
			for message in messages {
				let player_intention = self.get_update(message);
				intentions.push(player_intention);
			}
		}

		intentions
	}

	pub fn player_move(&self, action: actions::Move) -> PlayerIntention {
		PlayerIntention {
			player_id: self.id,
			intention: Intention::Move {
				direction: action.direction
			}
		}
	}

	pub fn get_client_id(&self) -> Uuid {
		self.net.get_id()
	}

	pub fn send(&self, message: &ServerMessage) {
		self.net.send(message);
	}
}
