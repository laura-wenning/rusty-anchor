// use rand::Rng;
use crate::map::*;
use crate::engine::GameEngine;
use crate::systems::*;

pub mod map;
pub mod components;
mod entities;
mod engine;
mod tiles;
mod systems;




fn main() {
  let mut engine: GameEngine = GameEngine::new();
  let camera_id: u32 = new_camera(&mut engine);
  new_player(&mut engine);
  engine.set_active_camera(camera_id);
  engine.draw();


  // let mut rng = rand::thread_rng();
  // let map = Map::new();
  // map.render();
  // println!("{:?}", map);

}

fn new_player(engine: &mut GameEngine) -> () {
  let player_id = engine.entities.add("player".to_string());
  engine.components.positions.register(player_id);
  if let Some(player_position) = engine.components.positions.get_mut(player_id) {
    (*player_position).x = 1f32;
    (*player_position).y = 1f32;
  }

  engine.components.visible.register(player_id);
  if let Some(player_visible) = engine.components.visible.get_mut(player_id) {
    (*player_visible).sprite = '@';
  }
}

fn new_camera(engine: &mut GameEngine) -> u32 {
  let camera_id = engine.entities.add("camera".to_string());
  engine.components.positions.register(camera_id);
  engine.components.cameras.register(camera_id);
  return camera_id;
}
