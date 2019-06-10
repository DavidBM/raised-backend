use crate::game::entities::domain::world::WorldHistory;
use crate::game::structs::Effect;
use std::fmt::Debug;


pub trait System: Debug + Send {
	fn execute_tick(&mut self, hola: &WorldHistory) -> Vec<Effect>;
}
