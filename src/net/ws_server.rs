use std::sync::mpsc;
use std::sync::mpsc::Sender;
use uuid;
use ws::listen;

use crate::game::structs::ClientActions;
use crate::net::{ClientPacket, GameClient, WsClient};

pub fn start(address: &str, waiting_queue: Sender<ClientActions>) {
    warn!("Server start on: {}", address);

    listen(address, |out| {
        let (packer_sender, packet_receiver) = mpsc::channel::<ClientPacket>();

        let id = uuid::Uuid::new_v4();

        let client = GameClient::new(id, out, packet_receiver);

        waiting_queue
            .send(ClientActions::New(client))
            .expect("Cannot send a client to the waiting queue");

        WsClient::new(id, packer_sender, waiting_queue.clone())
    })
    .expect("Cannot start ws server");
}
