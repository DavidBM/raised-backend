use crate::game::structs::{Effect, Intention};
use crate::game::entities::domain::pj::Pj;
use crate::config::player::SPEED;
use crate::game::engine::structs::Position;
use crate::game::entities::domain::world::WorldHistory;
use crate::game::entities::System;


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
	fn execute_tick(&mut self, world: &WorldHistory) -> Vec<Effect> {
		let world = world.get_current_inmutable();
		let mut players_positions: Vec<Effect> = Vec::new();

		for player in &world.players {

			let intention = match &player.intention {
				Some(intention) => intention,
				None => continue,
			};
			println!("Intetion {:?}", intention);
			match intention {
				Intention::Move{direction} => players_positions.push(Effect::PlayerMoved {
					player_id: player.id,
					position: self.get_next_position(&player, &direction, 0)
				}),
				_ => ()
			}
		}

		players_positions
	}
}
