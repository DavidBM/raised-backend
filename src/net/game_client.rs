use crate::game::structs::Effect;
use uuid::Uuid;
use std::sync::mpsc;
use crate::net::packets::ClientPacket;

use ws::Sender;

#[derive(Debug)]
pub struct GameClient {
	id: Uuid,
	client: Sender,
	receiver: mpsc::Receiver<ClientPacket>
}

impl GameClient {
	pub fn new(id: Uuid, client: Sender, receiver: mpsc::Receiver<ClientPacket>) -> GameClient {
		GameClient {id, receiver, client}
	}

	pub fn get_id(&self) -> Uuid {
		self.id
	}

	pub fn get_messages(&self) -> Option<Vec<ClientPacket>> {
		let mut messages: Vec<ClientPacket> = Vec::new();

		while let Ok(game_message) = self.receiver.try_recv() {
			messages.push(game_message);
		}

		trace!("Player packets: {:?}", messages);

		if !messages.is_empty() {
			Some(messages)
		}else{
			None
		}
	}

	pub fn send(&self, notification: &Effect) {
		let stringify_result = serde_json::to_string(notification);

		let serialized_notification = match stringify_result {
			Ok(result) => result,
			Err(error) => return trace!("{:?}", error),
		};

		let send_result = self.client.send(serialized_notification);

		match send_result{
			Ok(_) => trace!("Message send: {:?} {:?}", &self, notification),
			Err(error) => trace!("{:?} {:?}", &self, error)
		}


	}
}
