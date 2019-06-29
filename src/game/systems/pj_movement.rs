use crate::game::structs::{Effect, Intention};
use crate::game::domain::pj::Pj;
use crate::config::player::SPEED;
use crate::game::engine::structs::Position;
use crate::game::domain::world::WorldHistory;
use crate::game::systems::System;

#[derive(Debug)]
pub struct PjMovement;

impl PjMovement {
	fn get_next_position(&self, player: &Pj, direction: &f32, elapsed: u32) -> Position {
		let x = player.position.x + direction.cos() * SPEED as f32 * (elapsed as f32 / 1_000_000_f32);
		let y = player.position.y + direction.sin() * SPEED as f32 * (elapsed as f32 / 1_000_000_f32);

		Position{x: x, y: y, z: player.position.z}
	}
}

impl System for PjMovement {
	fn execute_tick(&mut self, world: &WorldHistory, elapsed: u32) -> Vec<Effect> {
		let world = world.get_current_inmutable();
		let mut players_positions: Vec<Effect> = Vec::new();

		let players = &world.read().expect(&format!("Cannot get read lock in Service {}", &self)).players;

		for player in players {

			player.intention.iter().for_each(|intention| {			
				trace!("PjMovement processing intention {:?} {:?}", player, intention);
				match intention {
					Intention::Move{direction} => players_positions.push(Effect::PlayerMoved {
						player_id: player.id,
						position: self.get_next_position(&player, &direction, elapsed)
					}),
					_ => ()
				}
			});
		}

		players_positions
	}
}

impl std::fmt::Display for PjMovement {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "PjMovement")
	}
}
