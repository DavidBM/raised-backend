use std::sync::mpsc::{Receiver};
use client;

#[derive(Debug)]
pub struct WaitingQueue {
	clients: Vec<client::GameClient>,
	receiver: Receiver<client::GameClient>
}

impl WaitingQueue {
	pub fn new(receiver: Receiver<client::GameClient>) -> WaitingQueue {
		let clients: Vec<client::GameClient> = Vec::new();
		WaitingQueue { clients:  clients, receiver: receiver }
	}

	pub fn wait_clients(&mut self) {
		loop {
			let client = self.receiver.recv().unwrap();
			self.add_client(client);
			self.check_clients();
		}
	}

	pub fn add_client(&mut self, client: client::GameClient) {
		loop {
			client.get_message();
		}
		//self.clients.push(client);
	}

	pub fn check_clients(&mut self) {
		if self.clients.len() < 4 { return }

		let mut players: Vec<client::GameClient> = Vec::new();

		for _ in 0..4 {
			players.push(self.clients.remove(0));
		}

		//TODO: Create game
	}
}
