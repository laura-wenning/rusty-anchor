use super::ComponentListTrait;

pub struct Controllable {
  owner: usize,
  is_active: bool,
}

impl ComponentListTrait for Controllable {
  fn register(entity_id: usize) -> Self {
    Self {
      owner: entity_id,
      is_active: true,
    }
  }
}