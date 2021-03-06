use crate::game::domain::world::WorldHistory;
use crate::game::structs::{Effect, Intention};
use crate::game::systems::System;

#[derive(Debug)]
pub struct PjConnection;

impl System for PjConnection {
    fn execute_tick(&mut self, world: &WorldHistory, _elapsed: u32) -> Vec<Effect> {
        let world = world.get_current_inmutable();
        let mut players_positions: Vec<Effect> = Vec::new();

        for player in &world.read().unwrap().players {
            player.intention.iter().for_each(|intention| {
                trace!("PjConnection processing intention {:?} {:?}", player, intention);

                match intention {
                    Intention::ConnectPlayer => players_positions.push(Effect::PlayerConnected(player.id)),
                    Intention::DisconnectPlayer => players_positions.push(Effect::PlayerDiconnected(player.id)),
                    _ => (),
                }
            });
        }

        players_positions
    }
}

impl std::fmt::Display for PjConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PjConnection")
    }
}
