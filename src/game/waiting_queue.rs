use std::sync::mpsc::{Receiver};
use game::structs::ClientActions;
use net;
use game::{Game, Map, Player};
use std::thread;

#[derive(Debug)]
pub struct WaitingQueue {
	clients: Vec<net::GameClient>,
	receiver: Receiver<ClientActions>
}

impl WaitingQueue {
	pub fn new(receiver: Receiver<ClientActions>) -> WaitingQueue {
		let clients: Vec<net::GameClient> = Vec::new();
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

	pub fn add_client(&mut self, client: net::GameClient) {
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

		let mut game = self.create_game();

		for _ in 0..4 {
			let client = self.clients.remove(0);
			let player = self.create_player(client);
			game.add_player(player);
		}

		thread::spawn(move || {
			game.start();
		});
	}

	fn create_game(&self) -> Game {
		let map = Map::new();
		Game::new(map)
	}

	fn create_player(&self, client: net::GameClient) -> Player {
		Player::new(client)
	}
}
