use crate::engine::GameEngine;

/// Handles translation of an entity
pub struct Translate {}
impl Translate {
  pub fn translate(engine: &mut GameEngine, entity_id: u32, x: f32, y: f32) {
    let translation = engine.components.translations.get_mut(entity_id);
    if let None = translation { return; }

    let translation = translation.unwrap();
    translation.x_position = translation.x_position + x;
    translation.y_position = translation.y_position + y;
  }
}