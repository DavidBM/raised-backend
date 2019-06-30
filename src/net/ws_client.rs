use std::sync::mpsc;
use ws;
use uuid::Uuid;
use serde_json;
use crate::net::*;
use crate::game::structs::ClientActions;

macro_rules! packet_decode {
	($($name:tt : $type:ty),*,) => {
		$(
			paste::item! {
				fn [<decode_ $name _message>](&self, packet: &str) {
					let decoded = {
						use packets::*;
						let decoded: Result<$type, _> = serde_json::from_str(packet);
						decoded
					};

					use ClientPacket::*;
					if let Ok(data) = decoded {
						self.sender.send($type(data)).expect(concat!("Cannot send to game client decoded message ", stringify!($name)));
					}
				}
			}
		)*
	};
}

macro_rules! packet_extract {
	($($name:tt),*,) => {
		paste::item! {
			fn extract_data(&self, message: packets::PacketType, packet: &str) {
				match message.t.as_ref() {
					$( stringify!($name) => self.[<decode_ $name _message>](packet), )*
					_ => self.extract_data_special_cases(message, packet),
				}
			}
		}
	};
}

macro_rules! implement_decoding {
	($($name:tt : $type:ty),*) => {
		packet_extract!( $($name,)*);
		packet_decode!( $($name:$type,)*);
	};
}

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

	/*pub fn get_id(&self) -> Uuid {
		self.id.clone()
	}*/

	pub fn proccess_message(&self, packet: String) {

		let text = packet.as_str();

		let decoded: Result<packets::PacketType, _> = serde_json::from_str(text);

		match decoded {
			Ok(data) => self.extract_data(data, text),
			Err(e) => info!("Not identified ws net package {:?}", e),
		}
	}

	fn extract_data_special_cases(&self, message: packets::PacketType, _packet: &str) {
		match message.t.as_ref() {
			"stay" => self.sender.send(ClientPacket::Stay).expect("Cannot send to game client decoded message stay"),
			_ => warn!("Not know message type: {:?}", message)
		}
	}

	implement_decoding!(
		equip: Equip,
		attack: Attack,
		login: Login,
		move: Move
	);
}

impl ws::Handler for WsClient {

	fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
		trace!("New connection: {:?}", &self);
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
			ws::CloseCode::Normal => trace!("The new client is done with the connection: {:?} {:?} {:?}", &self, code, reason),
			ws::CloseCode::Away   => trace!("The new client is leaving the site: {:?} {:?} {:?}", &self, code, reason),
			ws::CloseCode::Abnormal => trace!("Closing ws handshake failed! Unable to obtain closing status from client: {:?} {:?} {:?}", &self, code, reason),
			_ => trace!("The net client encountered an error: {:?} {:?} {:?}", &self, code, reason),
		}
	}

	fn on_error(&mut self, err: ws::Error) {
		trace!("The server encountered an error: {:?} {:?}", &self, err);
	}
}
