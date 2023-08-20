use crate::engine::GameEngine;

use super::Factory;

pub struct CameraFactory {}
impl Factory for CameraFactory {
  fn new(engine: &mut GameEngine) -> u32 {
    let camera_id = engine.entities.add("camera".to_string());

    engine.components.positions.register(camera_id);
    engine.components.cameras.register(camera_id);
    
    if let None = engine.active_camera_id {
      engine.set_active_camera(camera_id);
    }

    return camera_id;
  }
}