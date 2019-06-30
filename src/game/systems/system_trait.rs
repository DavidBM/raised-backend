use crate::game::domain::world::WorldHistory;
use crate::game::structs::Effect;
use std::fmt::Debug;
use std::fmt::Display;

pub trait System: Debug + Send + Display {
    fn execute_tick(&mut self, hola: &WorldHistory, elapsed: u32) -> Vec<Effect>;
}
