use std::sync::mpsc;
use net::Message as GameMessage;
use net::SendMessage;
use ws::Sender;

#[derive(Debug)]
pub struct GameClient {
	id: String,
	client: Sender,
	receiver: mpsc::Receiver<GameMessage>
}

impl GameClient {
	pub fn new(id: String, client: Sender, receiver: mpsc::Receiver<GameMessage>) -> GameClient {
		GameClient {id: id, receiver: receiver, client: client}
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

	pub fn send(&self, notification: &SendMessage) {
		let notification = notification.clone();
		let result = self.client.send("Hola! :D");
		match result{
			Ok(_) => println!("Message send: {:?}", notification),
			Err(error) => println!("{:?}", error)
		}
	}
}
