extern crate ws;
extern crate uuid;

use std::sync::mpsc;

use serde_json;
use net::*;
use net::Message as GameMessage;
use game::structs::ClientActions;


#[derive(Debug)]
pub struct WsClient {
	id: String,
	validated: bool,
	client: ws::Sender,
	sender: mpsc::Sender<GameMessage>,
	waiting_sender: mpsc::Sender<ClientActions>,
}

impl WsClient {
	pub fn new(client: ws::Sender, sender: mpsc::Sender<GameMessage>, waiting_sender: mpsc::Sender<ClientActions>) -> WsClient {
		WsClient {
			id: uuid::Uuid::new_v4().to_simple_string(),
			validated: false,
			client: client,
			sender: sender,
			waiting_sender: waiting_sender
		}
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn proccess_message(&self, packet: String) {

		let text = packet.as_str();

		let decoded: Result<MessageType, _> = serde_json::from_str(text);

		match decoded {
			Ok(data) => self.extract_data(data, text),
			Err(_) => println!("Not identify package"),
		}
	}

	fn extract_data(&self, message: MessageType, packet: &str) {
		match message.t.as_ref() {
			"login" => self.login_message(packet),
			"move" => self.move_message(packet),
			_ => println!("Not know message type: {}", message.t)
		}
	}

	fn login_message(&self, packet: &str) {
		let decoded: Result<LoginMessage, _> = serde_json::from_str(packet);

		if let Ok(data) = decoded {
			self.sender.send(GameMessage::LoginMessage(data)).unwrap();
		}
	}

	fn move_message(&self, packet: &str) {
		let decoded: Result<PlayerMove, _> = serde_json::from_str(packet);

		if let Ok(data) = decoded {
			self.sender.send(GameMessage::PlayerMove(data)).unwrap();
		}
	}
}

impl ws::Handler for WsClient {

	fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
		//println!("New connection...");
		Ok(())
	}

	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		if let Ok(message) = msg.into_text(){
			self.proccess_message(message)
		}

		Ok(())
	}

	fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
		let action = ClientActions::Delete(self.id.clone());
		self.waiting_sender.send(action).unwrap();

		self.sender.send(GameMessage::PlayerDisconnected).unwrap();

		match code {
			ws::CloseCode::Normal => println!("The client is done with the connection."),
			ws::CloseCode::Away   => println!("The client is leaving the site."),
			ws::CloseCode::Abnormal => println!("Closing handshake failed! Unable to obtain closing status from client."),
			_ => println!("The client encountered an error: {}", reason),
		}
	}

	fn on_error(&mut self, err: ws::Error) {
		println!("The server encountered an error: {:?}", err);
	}
}
