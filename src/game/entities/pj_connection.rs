use crate::game::entities::system_trait::System;
use crate::game::entities::domain::world::WorldHistory;
use crate::game::structs::{Effect, Intention};

#[derive(Debug)]
pub struct PjConnection;

impl System for PjConnection {
	fn execute_tick(&mut self, world: &WorldHistory, _elapsed: u32) -> Vec<Effect> {
		let world = world.get_current_inmutable();
		let mut players_positions: Vec<Effect> = Vec::new();

		for player in &world.read().unwrap().players {

			player.intention.iter().for_each(|intention| {			
				println!("Intetion {:?}", intention);
				match intention {
					Intention::ConnectPlayer => players_positions.push(Effect::PlayerConnected(player.id)),
					_ => ()
				}
			});
		}

		players_positions
	}
}
