use factories::Factory;
use factories::{
  camera::CameraFactory,
  player::PlayerFactory
};
use systems::input::Input;

// use rand::Rng;
use crate::engine::GameEngine;
use crate::systems::screen::Screen;

pub mod map;
pub mod components;
mod entities;
mod engine;
mod factories;
mod tiles;
mod systems;

use std::io;

fn main() -> io::Result<()> {
  if let Err(_) = setup() {
    teardown()?;
    return Ok(())
  }

  run_game()?;

  teardown()?;
  Ok(())
}

fn run_game() -> io::Result<()> {
  let mut engine: GameEngine = GameEngine::new();

  let player_id: u32 = PlayerFactory::new(&mut engine);
  CameraFactory::new(&mut engine);
  
  Screen::draw(&mut engine)?;

  for _ in 0..50 {
    Input::wait_for_keypress(&mut engine, player_id)?;
    Screen::draw(&mut engine)?;
  }
  Ok(())
}

fn setup() -> std::io::Result<()> {
  Screen::setup()?;
  Ok(())
}

fn teardown() -> std::io::Result<()> {
  Screen::teardown()?;
  Ok(())
}