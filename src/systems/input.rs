use crossterm::event::{read, Event, KeyCode};

use crate::engine::GameEngine;

use super::translate::Translate;

pub struct Input {}
impl Input {
  pub fn wait_for_keypress(engine: &mut GameEngine, player_id: u32) -> std::io::Result<()> {
    let mut command: Option<fn(&mut GameEngine, u32) -> ()> = None;
    loop {
      command = match read()? {
        Event::Key(event) => {
          Self::keypress_to_command(event.code)
        },
        _ => continue,
      };
      if let None = command { return Ok(()); }
      break;
    }

    command.unwrap()(engine, player_id);
    Ok(())
  }

  fn keypress_to_command(keypress: KeyCode) -> Option<fn(&mut GameEngine, u32) -> ()> {
    let run_action = match keypress {
      KeyCode::Left => Self::move_left,
      KeyCode::Right => Self::move_right,
      KeyCode::Up => Self::move_up,
      KeyCode::Down => Self::move_down,
      _ => return None,
    };

    return Some(run_action);
  }

  fn move_left(engine: &mut GameEngine, entity_id: u32) {
    Translate::translate(engine, entity_id, -1.0, 0.0);
  }

  fn move_right(engine: &mut GameEngine, entity_id: u32) {
    Translate::translate(engine, entity_id, 1.0, 0.0);
  }

  fn move_up(engine: &mut GameEngine, entity_id: u32) {
    Translate::translate(engine, entity_id, 0.0, -1.0);
  }

  fn move_down(engine: &mut GameEngine, entity_id: u32) {
    Translate::translate(engine, entity_id, 0.0, 1.0);
  }
}