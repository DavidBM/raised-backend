use crate::game::structs::Effect;
use crate::net::packets::ClientPacket;
use std::sync::mpsc;
use uuid::Uuid;

use ws::Sender;

#[derive(Debug)]
pub struct GameClient {
    id: Uuid,
    client: Sender,
    receiver: mpsc::Receiver<ClientPacket>,
}

impl GameClient {
    pub fn new(id: Uuid, client: Sender, receiver: mpsc::Receiver<ClientPacket>) -> GameClient {
        GameClient { id, receiver, client }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_messages(&self) -> Option<Vec<ClientPacket>> {
        let first_message = self.receiver.try_recv().ok()?;

        let mut messages: Vec<ClientPacket> = Vec::with_capacity(5);

        messages.push(first_message);

        while let Ok(game_message) = self.receiver.try_recv() {
            messages.push(game_message);
        }

        trace!("Player packets: {} messages with content {:?}", messages.len(), messages);

        Some(messages)
    }

    pub fn send(&self, notification: &Effect) {
        let stringify_result = serde_json::to_string(notification);

        let serialized_notification = match stringify_result {
            Ok(result) => result,
            Err(error) => return trace!("{:?}", error),
        };

        let send_result = self.client.send(serialized_notification);

        match send_result {
            Ok(_) => trace!("Message send: {:?} {:?}", &self, notification),
            Err(error) => trace!("Error sending message to client: {:?} {:?} {:?}", &self, notification, error),
        }
    }
}
