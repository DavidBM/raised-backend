use std::fmt::Display;
use crate::game::domain::world::WorldHistory;
use crate::game::structs::Effect;
use std::fmt::Debug;


pub trait System: Debug + Send + Display {
	fn execute_tick(&mut self, hola: &WorldHistory, elapsed: u32) -> Vec<Effect>;
}
