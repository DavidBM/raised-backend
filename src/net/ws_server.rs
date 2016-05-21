use uuid;
use ws::listen;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use net::{WsClient, GameClient, ClientPacket};
use game::structs::ClientActions;

pub fn start(address: &str, waiting_queue: Sender<ClientActions>) {

	listen(address, |out| {
		let (input_tx, input_rx): (Sender<ClientPacket>, Receiver<ClientPacket>) = mpsc::channel();

		let id = uuid::Uuid::new_v4();

		let ws_client = WsClient::new(id.clone() ,input_tx, waiting_queue.clone());

		let client = GameClient::new(id, out, input_rx);

		waiting_queue.send(ClientActions::New(client)).unwrap();

		ws_client
	}).unwrap()
}
