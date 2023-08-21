use crate::engine::GameEngine;

use super::Factory;

pub struct PlayerFactory {}
impl Factory for PlayerFactory {
  /// Creates a new player for the given Game Engine and returns the ID
  fn new(engine: &mut GameEngine) -> Result<u32, String> {
    let player_id = engine.entities.add("player".to_string());

    engine.components.positions.register(player_id);
    if let Some(player_position) = engine.components.positions.get_mut(player_id) {
      (*player_position).x = 1f32;
      (*player_position).y = 1f32;
    }

    engine.components.visible.register(player_id);
    
    if let Some(player_visible) = engine.components.visible.get_mut(player_id) {
      (*player_visible).sprite = '@';
      (*player_visible).is_visible = true;
    }

    engine.components.controllables.register(player_id);

    Ok(player_id)
  }
}