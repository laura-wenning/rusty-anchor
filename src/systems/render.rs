use std::convert::TryFrom;

use crate::components::camera::Camera;
use crate::engine::GameEngine;
// use crossterm::{execute, terminal::{SetSize}};

impl GameEngine {
  pub fn draw(&mut self) {
    // If there is no active camera, panic
    if let None = self.active_camera {
      return;
    }
    let camera_id = self.active_camera.unwrap();
    
    // TODO - validate that the camera is correct or assume?
    let camera_position = self.components.positions.get(camera_id).unwrap();
    let mut camera_camera = self.components.cameras.get_mut(camera_id).unwrap();

    initialize_camera(&mut camera_camera);
    flush_camera(&mut camera_camera);

    draw_to_screen(&mut camera_camera);
  }
}

fn initialize_camera(camera: &mut Camera) {
  let area = usize::try_from(camera.height * camera.width).unwrap();
  if camera.buffer.len() == area {
    return;
  }
  camera.buffer.resize(area, ' ');
}

fn flush_camera(camera: &mut Camera) {
  camera.buffer.fill('_');
}

fn draw_to_screen(camera: &mut Camera) {
  let mut screen = String::new();
  let area = usize::try_from(camera.height * camera.width).unwrap();
  let width = usize::try_from(camera.width).unwrap();
  for buffer_index in 0..area {
    if buffer_index % width == 0 && buffer_index != 0 {
      screen.push('\n');
    }

    screen.push(camera.buffer[buffer_index]);
  }
  println!("{}", screen);
}