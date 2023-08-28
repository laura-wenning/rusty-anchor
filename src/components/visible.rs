use super::ComponentListTrait;

pub struct Visible {
  pub owner: usize,
  pub is_visible: bool,
  pub sprite: char,
}

impl ComponentListTrait for Visible {
  fn register(entity_id: usize) -> Self {
    Self {
      owner: entity_id,
      is_visible: false,
      sprite: ' ',
    }
  }
}

