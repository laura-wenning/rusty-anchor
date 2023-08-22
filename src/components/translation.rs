use super::ComponentTrait;

pub struct Translation {
  pub owner: u32,

  pub x_position: f32,
  pub y_position: f32,
  pub z_position: f32, // Will be mostly used for layer heights

  pub x_scale: f32,
  pub y_scale: f32,
  pub z_scale: f32, // Possibly unused

  pub x_rotation: f32,
  pub y_rotation: f32,
  pub z_rotation: f32, // The only rotation that matters in 2d
}

impl ComponentTrait for Translation {
  fn new(entity_id: u32) -> Self {
    Self {
      owner: entity_id,

      x_position: 0.0,
      y_position: 0.0,
      z_position: 0.0,

      x_scale: 1.0,
      y_scale: 1.0,
      z_scale: 1.0,

      x_rotation: 0.0,
      y_rotation: 0.0,
      z_rotation: 0.0,
    }
  }
}

