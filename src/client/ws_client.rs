extern crate ws;
extern crate uuid;

use std::sync::mpsc;

use serde_json;
use ws::{Sender};
use client::{LoginMessage, MessageType, PlayerMove, Message as GameMessage};


#[derive(Debug)]
pub struct WsClient {
	id: String,
	validated: bool,
	client: Sender,
	sender: mpsc::Sender<Box<GameMessage + Send>>,
}

impl WsClient {
	pub fn new(client: Sender, sender: mpsc::Sender<Box<GameMessage + Send>>) -> WsClient {
		WsClient {
			id: uuid::Uuid::new_v4().to_simple_string(),
			validated: false,
			client: client,
			sender: sender
		}
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn proccess_message(&self, packet: String) -> () {

		let text = packet.as_str();

		let decoded: Result<MessageType, _> = serde_json::from_str(text);

		match decoded {
			Ok(data) => self.extract_data(data, text),
			Err(_) => {
				println!("Not identify package");
			},
		}
	}

	fn extract_data(&self, message: MessageType, packet: &str) -> () {
		match message.t.as_ref() {
			"login" => {
				let decoded: Result<LoginMessage, _> = serde_json::from_str(packet);

				match decoded {
					Ok(data) => println!("Loggin message received: {:?}", data),
					Err(_) => (),
				};
			},
			"move" => {
				let decoded: Result<PlayerMove, _> = serde_json::from_str(packet);

				match decoded {
					Ok(data) => {
						self.sender.send(Box::new(PlayerMove{x: data.x, y: data.y})).unwrap();
						()
					},
 						Err(_) => (),
 					};
 				},
			_ => println!("Not know message type: {}", message.t)
		}
	}
}
