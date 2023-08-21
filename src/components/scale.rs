use super::ComponentTrait;


pub struct Scale {
  owner: u32,
  width: f32,
  height: f32,
}

impl ComponentTrait for Scale {
  fn new(entity_id: u32) -> Self {
    Self {
      owner: entity_id,
      width: 1.0,
      height: 1.0,
    }
  }
}