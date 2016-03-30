use std::sync::mpsc;
use net::Message as GameMessage;

#[derive(Debug)]
pub struct GameClient {
	id: String,
	receiver: mpsc::Receiver<GameMessage>
}

impl GameClient {
	pub fn new(id: String, receiver: mpsc::Receiver<GameMessage>) -> GameClient {
		GameClient {id: id, receiver: receiver}
	}

	pub fn get_messages(&self) -> Option<Vec<GameMessage>> {
		let mut messages: Vec<GameMessage> = Vec::new();

		while let Ok(game_message) = self.receiver.try_recv() {
			messages.push(game_message);
		}

		if messages.len() > 0 {
			Some(messages)
		}else{
			None
		}
	}

	/*pub fn get_message(&self) -> Option<GameMessage> {
		let message = self.receiver.try_recv();

		if let Ok(message) = message {
			Some(message)
		}else {
		    None
		}
	}*/

	pub fn get_id(&self) -> String {
		self.id.clone()
	}
}
