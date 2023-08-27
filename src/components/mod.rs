use std::slice::Iter;

use crate::global::ENTITY_LIMIT;

use self::{
  camera::Camera,
  controllable::Controllable,
  translation::Translation,
  visible::Visible,
};

pub mod camera;
mod controllable;
pub mod translation;
mod visible;

pub trait ComponentTrait {
  fn new(entity_id: usize) -> Self;
}


// pub struct ComponentContainer< T> {
//   components: [T; ENTITY_LIMIT],
//   keys: Vec<usize>,
// }

// impl<'a, T: ComponentTrait> ComponentContainer<T> {
//   pub fn new() -> Self {
//     // const INIT: Option<T> = None;
//     Self {
//       components: [INIT; ENTITY_LIMIT], // Initializes all components as None to start
//       keys: Vec::new(),
//     }
//   }

//   /// Register a new component for a given entity ID
//   pub fn register(&mut self, entity_id: usize) -> Result<(), &str> {
//     if entity_id >= ENTITY_LIMIT { return Err("Entity ID is outside the bounds of the entity limit"); }

//     if let None = self.components[entity_id] {
//       return Err("Entity already exists");
//     }

//     let mut new_component: T = T::new(entity_id);
//     self.components[entity_id] = Some(&mut new_component);
//     return Ok(())
//   }

//   pub fn has(&self, entity_id: usize) -> bool {
//     if let None = self.components[entity_id] { return false; }
//     return true;
//   }

//   pub fn get(&self, entity_id: usize) -> Option<&T> {
//     let component = self.components[entity_id];
//     // We need to unwrap this to de and then re-reference the component
//     match self.components[entity_id] {
//       None => return None,
//       Some(component) => return Some(&*component),
//     }
//   }

//   pub fn get_mut(&mut self, entity_id: usize) -> Option<&mut T> {
//     self.components[entity_id]
//   }

//   pub fn iter(&mut self) -> Iter<Option<&mut T>> {
//     self.components.iter()
//   }

//   /// Gets a list of all keys for entities used within this component
//   /// 
//   /// The vector clone is used to prevent issues from lifetimes
//   /// TODO - possibly optimize this?
//   pub fn keys(&self) -> Vec<usize> {
//     self.keys.clone()
//   }
// }



pub struct ComponentManager {
  pub cameras: [Option<Camera>; ENTITY_LIMIT],
  pub controllables: [Option<Controllable>; ENTITY_LIMIT],
  pub translations: [Option<Translation>; ENTITY_LIMIT],
  pub visible: [Option<Visible>; ENTITY_LIMIT],
}



impl ComponentManager {
  pub fn new() -> Self {
    Self {
      cameras: [None; ENTITY_LIMIT],
      controllables: [None; ENTITY_LIMIT],
      translations: [None; ENTITY_LIMIT],
      visible: [None; ENTITY_LIMIT],
    } 
  }
}

