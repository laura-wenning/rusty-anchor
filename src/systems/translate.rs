use crate::engine::GameEngine;

/// Handles translation of an entity
pub struct Translate {}
impl Translate {
  pub fn translate(engine: &mut GameEngine, entity_id: u32, x: f32, y: f32) {
    let position = engine.components.positions.get_mut(entity_id);
    if let None = position { return; }

    let position = position.unwrap();
    position.x = position.x + x;
    position.y = position.y + y;
  }
}