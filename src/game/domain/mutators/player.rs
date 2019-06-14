use crate::game::domain::pj::Pj;
use crate::game::engine::structs::Position;
use crate::game::domain::world::World;

pub fn player_moved(world: &mut World, position: &Position, player_id: u64) {
	world.apply_to_player(&player_id, |player: &mut Pj| {
		player.position = position.clone();
	})
}
