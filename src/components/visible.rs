use super::ComponentTrait;

pub struct Visible {
  pub owner: usize,
  pub is_visible: bool,
  pub sprite: char,
}

impl ComponentTrait for Visible {
  fn new(entity_id: usize) -> Self {
    Self {
      owner: entity_id,
      is_visible: false,
      sprite: ' ',
    }
  }
}

