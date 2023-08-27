use super::ComponentTrait;

pub struct Camera {
  pub owner: usize,
  pub height: u32,
  pub width: u32,

  // Buffer is treated as a single string that will then be broken apart
  pub buffer: Vec<char>,
}

impl ComponentTrait for Camera {
  fn new(entity_id: usize) -> Self {
    Self {
      owner: entity_id,
      height: 5,
      width: 15,
      buffer: Vec::new(),
    }
  }
}

