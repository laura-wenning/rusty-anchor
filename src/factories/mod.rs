use crate::engine::GameEngine;

pub mod brush;
pub mod camera;
pub mod player;

pub trait Factory {
  fn new(engine: &mut GameEngine) -> Result<u32, String>;
}