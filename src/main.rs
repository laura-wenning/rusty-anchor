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
  let mut engine: GameEngine = setup_game();

  for _ in 0..50 {
    Screen::draw(&mut engine)?;
    Input::wait_for_keypress(&mut engine, 0)?;
  }
  Ok(())
}

/// Sets up the surrounding environment prior to the game running
fn setup() -> std::io::Result<()> {
  Screen::setup()?;
  Ok(())
}

/// Tears down all changes made to the surrounding environment following the conclusion of the game
fn teardown() -> std::io::Result<()> {
  Screen::teardown()?;
  Ok(())
}

/// Initializes the game space with all required defaults, menus, player, camera, and other entities
fn setup_game() -> GameEngine {
  let mut engine: GameEngine = GameEngine::new();
  PlayerFactory::new(&mut engine);
  CameraFactory::new(&mut engine);

  engine
}