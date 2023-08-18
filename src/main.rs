// use rand::Rng;
use crate::engine::GameEngine;
use crate::systems::screen::Screen;

pub mod map;
pub mod components;
mod entities;
mod engine;
mod tiles;
mod systems;

use std::{io, thread, time};


fn main() -> io::Result<()> {
  Screen::setup()?;
  let mut engine: GameEngine = GameEngine::new();
  let camera_id: u32 = new_camera(&mut engine);
  new_player(&mut engine);
  if let Err(_) = engine.set_active_camera(camera_id) {
    Screen::teardown()?;
    panic!("Camera could not be set. Exiting");
  }
  Screen::draw(&mut engine)?;

  thread::sleep(time::Duration::from_secs(5));
  // let mut rng = rand::thread_rng();
  // let map = Map::new();
  // map.render();
  // println!("{:?}", map);
  Screen::teardown()
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
    (*player_visible).is_visible = true;
  }
}

fn new_camera(engine: &mut GameEngine) -> u32 {
  let camera_id = engine.entities.add("camera".to_string());
  engine.components.positions.register(camera_id);
  engine.components.cameras.register(camera_id);
  return camera_id;
}
