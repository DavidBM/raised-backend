use std::sync::mpsc::{Receiver};
use crate::game::structs::ClientActions;
use uuid::Uuid;
use crate::net;
use crate::game::{Game, Player};
use std::thread;
use std::u64;

#[derive(Debug)]
pub struct WaitingQueue {
	clients: Vec<net::GameClient>,
	receiver: Receiver<ClientActions>,
	players_count: u64,
}

impl WaitingQueue {
	pub fn new(receiver: Receiver<ClientActions>) -> WaitingQueue {
		let clients: Vec<net::GameClient> = Vec::new();
		WaitingQueue { clients:  clients, receiver: receiver, players_count: 0 }
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

	pub fn remove_client(&mut self, id: Uuid) {
		let index = self.clients.iter().position(|r| r.get_id() == id);

		if let Some(index) = index {
			let client = self.clients.swap_remove(index);
			println!("Client removed from waiting queue: {:?}", client);
		}
	}

	pub fn check_clients(&mut self) {
		if self.clients.len() < 4 { return }

		self.create_game();
	}

	fn create_game(&mut self) {
		let mut game = Game::new();

		for _ in 0..4 {
			let client = self.clients.remove(0);
			let player = self.create_player(client);
			game.add_player(player);
		}

		thread::spawn(move || {
			game.start();
		});
	}

	fn create_player(&mut self, client: net::GameClient) -> Player {
		let player_id = self.players_count;
		self.players_count += 1;

		if self.players_count > u64::MAX - 10 { self.players_count = 0; }

		Player::new(client, player_id)
	}
}
