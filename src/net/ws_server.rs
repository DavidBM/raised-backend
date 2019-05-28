use uuid;
use ws::listen;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use net::{WsClient, GameClient, ClientPacket};
use game::structs::ClientActions;

pub fn start(address: &str, waiting_queue: Sender<ClientActions>) {

	println!("Listening webSocket connections");

	listen(address, |out| {
		let (packer_sender, packet_receiver): (Sender<ClientPacket>, Receiver<ClientPacket>) = mpsc::channel();

		let id = uuid::Uuid::new_v4();

		let client = GameClient::new(id, out, packet_receiver);

		waiting_queue.send(ClientActions::New(client)).unwrap();

		WsClient::new(id.clone() ,packer_sender, waiting_queue.clone())
	}).unwrap()
}
