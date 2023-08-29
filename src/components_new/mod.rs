use crate::global::ENTITY_LIMIT;

static mut POSITIONS: [Option<Position>; ENTITY_LIMIT] = [None; ENTITY_LIMIT];

#[derive(Clone)]
struct Position {
  x: f32,
  y: f32,
  z: f32,
}

impl Position {
  pub fn new() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0
    }
  }
}

struct Positions {}
impl Positions {
  /// Registers an entity_id for the appropriate component
  pub fn register(entity_id: usize) -> Result<(),()> {
    if !Self::can_register(entity_id) { return Err(()); }

    Ok(())
  }

  /// Gets a non-mutable position object, if registered
  pub fn get(entity_id: usize) -> Option<Position> {
    if is_entity_id_out_of_bounds(entity_id) { return None; }
    let mut position: Option<Position> = None;
    unsafe {
      position = POSITIONS[entity_id];
    }
    position
  }

  pub fn set(entity_id: usize, position: Position) -> Result<(), ()> {
    if is_entity_id_out_of_bounds(entity_id) { return Err(()); }
    else if !Self::is_registered(entity_id) { return Err(())}
    Ok(())
  }

  /// Checks if the given entity_id is already registered
  pub fn is_registered(entity_id: usize) -> bool {
    let position = Self::get(entity_id);
    if let None = position { return false; }
    true
  }

  /// Determines if a given entity_id can be registered for the appropriate component
  fn can_register(entity_id: usize) -> bool {
    if is_entity_id_out_of_bounds(entity_id) { return false; }
    if Self::is_registered(entity_id) { return false; }
    true
  }


  

  
}

/// Determines if the entity ID is outside of the bounds of the ENTITY_LIMIT
fn is_entity_id_out_of_bounds(entity_id: usize) -> bool {
  if entity_id >= ENTITY_LIMIT { return true; }
  false
}