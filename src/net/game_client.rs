use std::sync::mpsc;
use net::packets::ClientPacket;
use net::packets::ServerMessage;
use ws::Sender;

#[derive(Debug)]
pub struct GameClient {
	id: String,
	client: Sender,
	receiver: mpsc::Receiver<ClientPacket>
}

impl GameClient {
	pub fn new(id: String, client: Sender, receiver: mpsc::Receiver<ClientPacket>) -> GameClient {
		GameClient {id: id, receiver: receiver, client: client}
	}

	pub fn get_messages(&self) -> Option<Vec<ClientPacket>> {
		let mut messages: Vec<ClientPacket> = Vec::new();

		while let Ok(game_message) = self.receiver.try_recv() {
			messages.push(game_message);
		}

		if messages.len() > 0 {
			Some(messages)
		}else{
			None
		}
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn send(&self, notification: &ServerMessage) {
		let notification = notification.clone();
		let result = self.client.send("Hola! :D");
		match result{
			Ok(_) => println!("Message send: {:?}", notification),
			Err(error) => println!("{:?}", error)
		}
	}
}
