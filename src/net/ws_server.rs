use uuid;
use ws::listen;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use crate::net::{WsClient, GameClient, ClientPacket};
use crate::game::structs::ClientActions;

pub fn start(address: &str, waiting_queue: Sender<ClientActions>) {

	warn!("Server start on: {}", address);

	listen(address, |out| {
		let (packer_sender, packet_receiver): (Sender<ClientPacket>, Receiver<ClientPacket>) = mpsc::channel();

		let id = uuid::Uuid::new_v4();

		let client = GameClient::new(id, out, packet_receiver);

		waiting_queue.send(ClientActions::New(client)).expect("Cannot send a client to the waiting queue");

		WsClient::new(id ,packer_sender, waiting_queue.clone())
	}).expect("Cannot start ws server");
}
