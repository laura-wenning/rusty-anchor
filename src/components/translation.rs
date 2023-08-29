use crate::global::ENTITY_LIMIT;

use super::{ComponentListTrait, ComponentTrait};

static mut TRANSLATIONS: [Option<Translation>; ENTITY_LIMIT] = [None; ENTITY_LIMIT];

#[derive(Clone)]
pub struct Coordinates {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Coordinates {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }
}

#[derive(Clone)]
pub struct Translation {
  pub position: Coordinates,
  pub origin: Coordinates, // The translation of the point of origin from the center of this object
  pub scale: Coordinates,

}

impl ComponentTrait for Translation {
  fn new() -> Self {
    Self {
      position: Coordinates::new(0.0, 0.0, 0.0),
      scale: Coordinates::new(1.0, 1.0, 1.0),
      origin: Coordinates::new(0.0, 0.0, 0.0),
    }
  }
}

pub struct Translations {}



