/// EntityAPI defines the API used for the entities such that we can always ensure that 
/// our other code will not need to change for major changes to the Entity object
/// 
pub trait EntityEngineAPI: EntityEngineAPIClone {
  /// Any setup functionality required
  fn setup(&self) -> ();
  /// Any teardown functionality required
  fn teardown(&self) -> ();
  /// Register a new entity
  fn register(&self) -> Result<usize, String>;
  /// Frees a pre-existing entity
  fn free(&self, entity_id: usize) -> Result<(), String>;
}


trait EntityEngineAPIClone {
  fn clone_box(&self) -> Box<dyn EntityEngineAPI>;
}

impl<T> EntityEngineAPIClone for T
where
  T: 'static + EntityEngineAPI + Clone,
{
  fn clone_box(&self) -> Box<dyn EntityEngineAPI> {
    Box::new(self.clone())
  }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn EntityEngineAPI> {
  fn clone(&self) -> Box<dyn EntityEngineAPI> {
    self.clone_box()
  }
}
