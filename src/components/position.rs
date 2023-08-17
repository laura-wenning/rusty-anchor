use super::ComponentTrait;

pub struct Position {
  pub owner: u32,
  pub x: f32,
  pub y: f32,
}


impl ComponentTrait for Position {
  fn new(entity_id: u32) -> Self {
    Self {
      owner: entity_id,
      x: 0f32,
      y: 0f32,
    }
  }
}

