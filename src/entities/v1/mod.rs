use crate::global::ENTITY_LIMIT;

use super::api::EntityEngineAPI;

static mut ENTITY_LIST: Vec<usize> = vec![];
static mut AVAILABLE_ENTITIES: Vec<usize> = vec![];

#[derive(Clone)]
pub struct EntityEngineV1 {}

impl EntityEngineV1 {
  pub fn new() -> Self {
    Self {}
  }
}

impl EntityEngineAPI for EntityEngineV1 {
  /// Initializes the entities
  fn setup(&self) {
    unsafe {
      for entity_id in 0..ENTITY_LIMIT {
        AVAILABLE_ENTITIES.push(entity_id);
      }
    }
  }

  fn teardown(&self) {

  }

  /// Registers a new entity
  fn register(&self) -> Result<usize, String> {
    let mut next_available_entity: Option<&usize> = None;
    unsafe {
      next_available_entity = AVAILABLE_ENTITIES.first();
    }

    if let None = next_available_entity { return Err("Entity limit exceeded!".to_string()); }
    let next_available_entity: usize = *next_available_entity.unwrap();

    unsafe {
      if ENTITY_LIST.contains(&next_available_entity) { return Err("Entity is already registered".into()); }
      ENTITY_LIST.push(next_available_entity);
      AVAILABLE_ENTITIES.remove(0);
    }

    Ok(next_available_entity)
  }

  fn free(&self, entity_id: usize) -> Result<(), String> {
    
    Ok(())
  }
}