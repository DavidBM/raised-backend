use game::entities::domain::world::WorldHistory;
use game::structs::PlayerIntention;
use crate::game::structs::IntentionEffect;
use crate::game::entities::System;


#[derive(Debug)]
pub struct PjMovement;

impl System for PjMovement {
	fn execute_tick(&mut self, _hola: &WorldHistory, _intention: Vec<PlayerIntention>) -> IntentionEffect {
		unimplemented!();
	}
}
