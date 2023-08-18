use std::convert::TryFrom;
use std::io::{self, Stdout, Write};

use crate::components::camera::Camera;
use crate::engine::GameEngine;
use crossterm::style::Stylize;
use crossterm::{QueueableCommand, style};
use crossterm::{execute, {
  style::ResetColor,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
  cursor::{self, Show, Hide}
}};


mod validation;

pub struct Screen {}
impl Screen {
  pub fn setup() -> io::Result<()> {
    execute!(
      io::stdout(),
      EnterAlternateScreen,
      Hide,
    )
  }

  pub fn teardown() -> io::Result<()> {
    execute!(
      io::stdout(),
      Show,
      ResetColor,
      LeaveAlternateScreen
    )
  }

  pub fn draw(engine: &mut GameEngine) -> io::Result<()> {
    if let Err(error_message) = Self::can_draw(engine) {
      eprintln!("{}", error_message);
      return Ok(());
    }

    let ref active_camera_id = engine.active_camera_id.unwrap();
    let mut camera_camera = engine.components.cameras.get_mut(*active_camera_id).unwrap();

    let mut stdout = io::stdout();
    Self::draw_background(&mut stdout, camera_camera)?;
    Self::draw_entities(&mut stdout, engine)?;
    
    stdout.flush()?;

    // initialize_camera(&mut camera_camera);
    // flush_camera(&mut camera_camera);

    // draw_to_screen(&mut camera_camera);

    Ok(())
  }

  fn draw_background(stdout: &mut Stdout, camera: &Camera) -> io::Result<()> {
    for y_position in 0..camera.height {
      for x_position in 0..camera.width {
        Self::draw_at(stdout, x_position, y_position, '_')?;
      }
    }
    Ok(())
  }

  fn draw_entities(stdout: &mut Stdout, engine: &mut GameEngine) -> io::Result<()> {
    let mut iterator = engine.components.visible.iter();
    let camera = engine.components.cameras.get(engine.active_camera_id.unwrap()).unwrap();
    loop {
      let visible = iterator.next();
      if let None = visible { break Ok(()); }

      let (entity_id, visible) = visible.unwrap();
      if !visible.is_visible { continue; }
      let position = engine.components.positions.get(*entity_id);
      if let None = position { continue; }
      let position = position.unwrap();

      // TODO - bounding system
      if position.x < 0.0 || position.x > (camera.width - 1) as f32 { continue; }
      else if position.y < 0.0 || position.y > (camera.height - 1) as f32 { continue; }

      let x = position.x.round() as u32;
      let y = position.y.round() as u32;

      Self::draw_at(stdout, x, y, visible.sprite)?;
    }
  }

  fn draw_at(stdout: &mut Stdout, x: u32, y: u32, sprite: char) -> io::Result<()> {
    stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
    stdout.queue(style::PrintStyledContent(sprite.white()))?;
    Ok(())
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