use crate::{engine::GameEngine, components::translation::{Coordinates, Translation}, systems::translate};

pub mod brush;
pub mod camera;
pub mod player;

pub trait Factory {
  fn new(engine: &mut GameEngine) -> Result<u32, String>;
}

pub struct GenericFactory<'a> {
  pub engine: &'a mut GameEngine,
  pub entity_id: u32,
}

impl<'a> GenericFactory<'a> {
  pub fn new(engine: &'a mut GameEngine, name: String) -> Self {
    let entity_id = engine.entities.add(name);
    Self {
      engine,
      entity_id
    }
  }

  /// Finishes the build
  pub fn build(&'a mut self) -> &'a mut GameEngine {
    self.engine
  }

  // pub fn translation(
  //   &'a mut self,
  //   position: Option<Coordinates>,
  //   scale: Option<Coordinates>,
  //   origin: Option<Coordinates>
  // ) -> &'a mut Self {
  //   self.engine.components.translations.register(self.entity_id);

  //   let translation = self.engine.components.translations.get_mut(self.entity_id);
  //   if let None = translation { return self; }
  //   let translation = translation.unwrap();

  //   if let Some(unwrapped_position) = position { translation.position = unwrapped_position; }
  //   if let Some(unwrapped_scale) = scale { translation.scale = unwrapped_scale; }
  //   if let Some(unwrapped_origin) = origin { translation.origin = unwrapped_origin; }

  //   self
  // }
}

impl <'a> GenericFactory<'a> {
  pub fn translation(&'a mut self) -> TranslationFactory {
    if !self.engine.components.translations.has(self.entity_id) {
      self.engine.components.translations.register(self.entity_id);
    }

    
    TranslationFactory::<'a>::new(self)
  }
}

pub struct TranslationFactory<'a> {
  pub factory: &'a mut GenericFactory<'a>,
  pub translation: Option<&'a mut Translation>
}

impl<'a> TranslationFactory<'a> {
  pub fn new(factory: &'a mut GenericFactory) -> Self {

    let mut newSelf = Self {
      factory,
      translation: None
    };
    newSelf.load();
    newSelf
  }

  pub fn load(&mut self) {
    self.translation = self.factory.engine.components.translations.get_mut(self.factory.entity_id);
  }

  pub fn close(&mut self) -> &'a GenericFactory {
    return self.factory;
  }

  pub fn position(&'a mut self, position: Coordinates) -> &'a Self {
    if let None = self.translation { return self; }
    self.translation.unwrap().position = position;
    self
  }

  pub fn scale(&'a mut self, scale: Coordinates) -> &'a Self {
    if let None = self.translation { return self; }
    self.translation.scale = scale;
    self
  }

  pub fn origin(&'a mut self, origin: Coordinates) -> &'a Self {
    if let None = self.translation { return self; }
    self.translation.origin = origin;
    self
  }
}