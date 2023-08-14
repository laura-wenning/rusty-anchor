// use rand::Rng;
use crate::map::*;

pub mod map;

fn main() {
  // let mut rng = rand::thread_rng();
  let map = Map::new();
  map.render();
  // println!("{:?}", map);

}


// struct Engine {

// }

// struct Screen {
//   position: Position,
//   width: i32,
//   height: i32
// }

// // Describes a room within a map
// struct Room {
//   position: Position,
//   width: i32,
//   height: i32,
// }

// impl Room {
//   fn new() {
//     Room {
//       position: Position()
//     }
//   }
// }

// type Postion(i32, i32);