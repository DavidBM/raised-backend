use game::entities::domain::world::WorldHistory;
use game::structs::PlayerIntention;
use game::structs::IntentionEffect;
use std::fmt::Debug;


pub trait System: Debug + Send {
	fn execute_tick(&mut self, hola: &WorldHistory, _intentions: Vec<PlayerIntention>) -> IntentionEffect;
}
