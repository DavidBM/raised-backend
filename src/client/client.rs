extern crate ws;
extern crate uuid;

use serde_json;
use ws::{Sender};
use client::LoginMessage;
use client::MessageType;


#[derive(Debug)]
pub struct WsClient {
	id: String,
	validated: bool,
	client: Sender,
}

impl WsClient {
	pub fn new(client: Sender) -> WsClient {
		WsClient {
			id: uuid::Uuid::new_v4().to_simple_string(),
			validated: false,
			client: client
		}
	}

	pub fn proccess_message(&self, packet: String) -> () {

		let text = packet.as_str();

		let decoded: Result<MessageType, _> = serde_json::from_str(text);

		match decoded {
			Ok(data) => self.extract_data(data, text),
			Err(_) => (),
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
			_ => println!("Not know message type: {}", message.t)
		}
	}
}
