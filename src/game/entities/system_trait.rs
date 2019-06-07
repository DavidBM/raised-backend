use game::entities::domain::world::WorldHistory;
use game::structs::PlayerIntention;
use game::structs::Effect;
use std::fmt::Debug;


pub trait System: Debug + Send {
	fn execute_tick(&mut self, hola: &WorldHistory) -> Vec<Effect>;
}
