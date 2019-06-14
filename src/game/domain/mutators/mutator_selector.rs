use crate::game::domain::mutators::*;
use crate::game::domain::world::World;
use crate::game::structs::Effect;


pub fn apply_effects(effects: &Vec<Effect>, mut world: &mut World) {
	for effect in effects.iter() {
		match effect {
			Effect::PlayerMoved{position, player_id} => player_moved(&mut world, position, *player_id),
			Effect::PlayerConnected(_player_id) => (),
			Effect::PlayerDiconnected(_player_id) => (),
		}
	}
}
