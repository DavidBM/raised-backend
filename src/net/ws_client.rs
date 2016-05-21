use std::sync::mpsc;
use ws;
use uuid::Uuid;

use serde_json;
use net::*;
use game::structs::ClientActions;


#[derive(Debug)]
pub struct WsClient {
	id: Uuid,
	validated: bool,
	sender: mpsc::Sender<ClientPacket>,
	waiting_sender: mpsc::Sender<ClientActions>,
}

impl WsClient {
	pub fn new(id: Uuid, sender: mpsc::Sender<ClientPacket>, waiting_sender: mpsc::Sender<ClientActions>) -> WsClient {
		WsClient {
			id: id,
			validated: false,
			sender: sender,
			waiting_sender: waiting_sender
		}
	}

	pub fn get_id(&self) -> Uuid {
		self.id.clone()
	}

	pub fn proccess_message(&self, packet: String) {

		let text = packet.as_str();

		let decoded: Result<packets::PacketType, _> = serde_json::from_str(text);

		match decoded {
			Ok(data) => self.extract_data(data, text),
			Err(e) => println!("Not identify package {:?}", e),
		}
	}

	fn extract_data(&self, message: packets::PacketType, packet: &str) {
		match message.t.as_ref() {
			"login" => self.login_message(packet),
			"move" => self.move_message(packet),
			"stay" => self.sender.send(ClientPacket::Stay).unwrap(),
			"attack" => self.attack_message(packet),
			"equip" => self.equip_message(packet),
			_ => println!("Not know message type: {:?}", message)
		}
	}

	fn login_message(&self, packet: &str) {
		let decoded: Result<packets::Login, _> = serde_json::from_str(packet);

		if let Ok(data) = decoded {
			self.sender.send(ClientPacket::Login(data)).unwrap();
		}
	}

	fn move_message(&self, packet: &str) {
		let decoded: Result<packets::Move, _> = serde_json::from_str(packet);

		if let Ok(data) = decoded {
			self.sender.send(ClientPacket::Move(data)).unwrap();
		}
	}

	fn attack_message(&self, packet: &str) {
		let decoded: Result<packets::Attack, _> = serde_json::from_str(packet);

		if let Ok(data) = decoded {
			self.sender.send(ClientPacket::Attack(data)).unwrap();
		}
	}

	fn equip_message(&self, packet: &str) {
		let decoded: Result<packets::Equip, _> = serde_json::from_str(packet);

		if let Ok(data) = decoded {
			self.sender.send(ClientPacket::Equip(data)).unwrap();
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

		self.sender.send(ClientPacket::Disconnected).unwrap();

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
