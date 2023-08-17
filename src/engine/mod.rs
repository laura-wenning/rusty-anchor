use crate::entities::EntityManager;
use crate::components::ComponentManager;

pub struct GameEngine {
  pub entities: EntityManager,
  pub components: ComponentManager,

  pub active_camera: Option<u32>,
}

impl GameEngine {
  pub fn new() -> Self {
    Self {
      entities: EntityManager::new(),
      components: ComponentManager::new(),

      active_camera: None
    }
  }

  pub fn set_active_camera(&mut self, camera_id: u32) -> bool {
    // If this is not a valid camera, fail
    if let None = self.components.cameras.get(camera_id) {
      return false;
    }
    self.active_camera = Some(camera_id);
    return true;
  }

 
}
