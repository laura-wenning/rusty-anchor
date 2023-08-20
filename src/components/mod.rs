use std::collections::{HashMap, hash_map::Iter};

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
}

impl<T: ComponentTrait> ComponentContainer<T> {
  pub fn new() -> Self {
    Self {
      components: HashMap::new(),
    }
  }

  pub fn register(&mut self, entity_id: u32) -> () {
    let new_component: T = T::new(entity_id);
    self.components.insert(entity_id, new_component);
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

