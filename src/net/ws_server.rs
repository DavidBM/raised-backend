use ws::listen;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use net::{WsClient, GameClient, Message as GameMessage};
use game::WaitingQueue;
use game::structs::ClientActions;

pub fn start(address: &str) {
	let (tx_wq, rx_wq): (Sender<ClientActions>, Receiver<ClientActions>) = mpsc::channel();

	thread::spawn(move || {
		let mut waiting_queue = WaitingQueue::new(rx_wq);
		waiting_queue.wait_clients();
	});

	listen(address, |out| {
		let (input_tx, input_rx): (Sender<GameMessage>, Receiver<GameMessage>) = mpsc::channel();

		let ws_client = WsClient::new(input_tx, tx_wq.clone());

		let id = ws_client.get_id();

		let client = GameClient::new(id, out, input_rx);

		tx_wq.send(ClientActions::New(client)).unwrap();

		ws_client
	}).unwrap()
}
