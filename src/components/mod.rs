use std::collections::{HashMap, hash_map::{Iter, Keys}};

use self::{
  camera::Camera,
  controllable::Controllable,
  position::Position,
  visible::Visible
};

pub mod camera;
mod controllable;
mod position;
mod visible;

pub trait ComponentTrait {
  fn new(entity_id: u32) -> Self;
}


pub struct ComponentContainer<T> {
  components: HashMap<u32, T>,
  keys: Vec<u32>,
}

impl<T: ComponentTrait> ComponentContainer<T> {
  pub fn new() -> Self {
    Self {
      components: HashMap::new(),
      keys: Vec::new(),
    }
  }

  pub fn register(&mut self, entity_id: u32) -> () {
    let new_component: T = T::new(entity_id);
    self.components.insert(entity_id, new_component);
    self.keys.push(entity_id);
  }

  pub fn get(&self, entity_id: u32) -> Option<&T> {
    self.components.get(&entity_id)
  }

  pub fn get_mut(&mut self, entity_id: u32) -> Option<&mut T> {
    self.components.get_mut(&entity_id)
  }

  pub fn iter(&mut self) -> Iter<u32, T> {
    self.components.iter()
  }

  /// Gets a list of all keys for entities used within this component
  /// 
  /// The vector clone is used to prevent issues from lifetimes
  /// TODO - possibly optimize this?
  pub fn keys(&self) -> Vec<u32> {
    self.keys.clone()
  }
}



pub struct ComponentManager {
  pub cameras: ComponentContainer<Camera>,
  pub controllables: ComponentContainer<Controllable>,
  pub positions: ComponentContainer<Position>,
  pub visible: ComponentContainer<Visible>,
}



impl ComponentManager {
  pub fn new() -> Self {
    Self {
      cameras: ComponentContainer::new(),
      controllables: ComponentContainer::new(),
      positions: ComponentContainer::new(),
      visible: ComponentContainer::new(),
    } 
  }
}

