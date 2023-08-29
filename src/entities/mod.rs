use self::api::EntityEngineAPI;

mod api;
pub mod v1;

static mut ENTITY_ENGINE: Option<Box<dyn EntityEngineAPI>> = None;

pub struct Entities {}
impl Entities {
  /// Sets up the entity engine to use for this game instance
  /// Uses dependency injection to allow for the implementations to be swapped easily if needed
  pub fn setup(entity_api: impl EntityEngineAPI + 'static) -> Result<(), String> {
    unsafe {
      if let Some(_) = ENTITY_ENGINE {
        return Err("An entity engine is already loaded".into());
      }
    }
    entity_api.setup();
    unsafe {
      ENTITY_ENGINE = Some(Box::new(entity_api));
    }
    Ok(())
  }

  fn get() -> Option<Box<dyn EntityEngineAPI>> {
    let mut engine: Option<Box<dyn EntityEngineAPI>> = None;
    unsafe {
      if let None = ENTITY_ENGINE { return None; }
      let new_engine = ENTITY_ENGINE.clone().unwrap();
      engine = Some(new_engine);
    }
    engine
  }

  pub fn register() -> Result<usize, String> {
    let engine = Self::get();
    if let None = engine { return Err("No entity engine is loaded".into()); }
    let engine = engine.unwrap();
    engine.register()
  }
}

pub struct Entity {
  id: usize,
  name: String,
}

pub struct EntityManager {
  current_index: usize,
  entities: Vec<Entity>,
}

impl EntityManager {
  pub fn new() -> Self {
    Self {
      current_index: 0,
      entities: Vec::new()
    }
  }

  pub fn add(&mut self, name: String) -> usize {
    let new_id = self.current_index.clone();
    self.current_index += 1;

    let new_entity = Entity {
      id: new_id.clone(),
      name,
    };
    self.entities.push(new_entity);

    new_id
  }
}