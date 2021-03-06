use crate::game::structs::ClientActions;
use crate::game::{GameManager, Player};
use crate::net;
use std::sync::mpsc::Receiver;
use std::thread;
use std::u64;
use uuid::Uuid;

#[derive(Debug)]
pub struct WaitingQueue {
    clients: Vec<net::GameClient>,
    receiver: Receiver<ClientActions>,
    players_count: u64,
    game_count: u64,
}

impl WaitingQueue {
    pub fn new(receiver: Receiver<ClientActions>) -> WaitingQueue {
        let clients: Vec<net::GameClient> = Vec::new();
        WaitingQueue {
            clients,
            receiver,
            players_count: 0,
            game_count: 0,
        }
    }

    pub fn wait_clients(&mut self) {
        loop {
            let client = self.receiver.recv().unwrap();
            match client {
                ClientActions::New(client) => {
                    self.add_client(client);
                    self.check_clients();
                }
                ClientActions::Delete(id) => {
                    self.remove_client(id);
                }
            }
        }
    }

    pub fn add_client(&mut self, client: net::GameClient) {
        info!("Client added to waiting queue. ID: {:?}", client);
        self.clients.push(client);
    }

    pub fn remove_client(&mut self, id: Uuid) {
        let index = self.clients.iter().position(|r| r.get_id() == id);

        if let Some(index) = index {
            let client = self.clients.swap_remove(index);
            info!("Client removed from waiting queue: {:?}", client);
        }
    }

    pub fn check_clients(&mut self) {
        if self.clients.len() >= 4 {
            self.create_game();
        }
    }

    fn create_game(&mut self) {
        let mut game = GameManager::new();

        for _ in 0..4 {
            let client = self.clients.swap_remove(0);
            let player = self.create_player(client);
            game.add_player(player);
        }

        let game_id = self.get_game_id();

        let _ = thread::Builder::new()
            .name(format!("Game {}", game_id))
            .spawn(move || {
                game.start();
            })
            .map_err(|err| error!("Error creating a game {} (thread): {:?}", game_id, err));
    }

    fn create_player(&mut self, client: net::GameClient) -> Player {
        let player_id = self.players_count;
        self.players_count += 1;

        if self.players_count == u64::MAX {
            self.players_count = 0;
        }

        Player::new(client, player_id)
    }

    fn get_game_id(&mut self) -> u64 {
        let game_id = self.game_count;
        self.game_count += 1;

        if self.game_count == u64::MAX {
            self.game_count = 0;
        }

        game_id
    }
}
