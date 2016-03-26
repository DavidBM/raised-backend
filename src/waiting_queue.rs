use std::sync::mpsc::{Receiver};
use client;

#[derive(Debug)]
pub enum ClientActions {
	New(client::GameClient),
	Delete(String)
}

#[derive(Debug)]
pub struct WaitingQueue {
	clients: Vec<client::GameClient>,
	receiver: Receiver<ClientActions>
}

impl WaitingQueue {
	pub fn new(receiver: Receiver<ClientActions>) -> WaitingQueue {
		let clients: Vec<client::GameClient> = Vec::new();
		WaitingQueue { clients:  clients, receiver: receiver }
	}

	pub fn wait_clients(&mut self) {
		loop {
			let client = self.receiver.recv().unwrap();
			match client {
				ClientActions::New(client) => {
					self.add_client(client);
					self.check_clients();
				},
				ClientActions::Delete(id) => {
					self.remove_client(id);
				},
			}
		}
	}

	pub fn add_client(&mut self, client: client::GameClient) {
		println!("Client added to waiting queue. ID: {:?}", client);
		self.clients.push(client);
	}

	pub fn remove_client(&mut self, id: String) {
		let index = self.clients.iter().position(|r| r.get_id() == id);

		if let Some(index) = index {
			let client = self.clients.swap_remove(index);
			println!("Client removed from waiting queue: {:?}", client);
		}
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
