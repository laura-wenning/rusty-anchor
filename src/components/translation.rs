use super::ComponentTrait;

pub struct Coordinates {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Coordinates {
  fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }
}

pub struct Translation {
  pub owner: u32,

  pub position: Coordinates,
  pub origin: Coordinates,
  pub scale: Coordinates,

}

impl ComponentTrait for Translation {
  fn new(entity_id: u32) -> Self {
    Self {
      owner: entity_id,

      position: Coordinates::new(0.0, 0.0, 0.0),
      scale: Coordinates::new(0.0, 0.0, 0.0),
      origin: Coordinates::new(0.0, 0.0, 0.0),
    }
  }
}

