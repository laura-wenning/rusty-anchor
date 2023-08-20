use crate::engine::GameEngine;

pub mod camera;
pub mod player;

pub trait Factory {
  fn new(engine: &mut GameEngine) -> u32;
}