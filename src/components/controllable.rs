use super::ComponentTrait;

pub struct Controllable {
  owner: usize,
  is_active: bool,
}

impl ComponentTrait for Controllable {
  fn new(entity_id: usize) -> Self {
    Self {
      owner: entity_id,
      is_active: true,
    }
  }
}