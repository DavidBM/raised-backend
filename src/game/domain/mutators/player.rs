use crate::game::domain::world::World;
use crate::game::engine::structs::Position;

pub fn player_moved(world: &mut World, position: &Position, player_id: u64) {
    world.apply_to_player(player_id, |player| player.position = position.clone());
}

pub fn player_disconected(world: &mut World, player_id: u64) {
    world.players.retain(|player| player.id != player_id)
}
