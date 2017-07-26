extern crate rustyline;
extern crate omn_labs;
extern crate specs;

use omn_labs::state::{State, StateMachine, Trans};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use specs::World;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Room {
    description: String,
    exits: HashMap<Direction, Rc<Room>>,
}

impl Room {
    pub fn new(description: &str, exits: HashMap<Direction, Rc<Room>>) -> Room {
        Room {
            description: description.to_owned(),
            exits: exits.clone(),
        }
    }
}

impl State for Room {
    fn update(&mut self, _: &mut World) -> Trans {
        Trans::None
    }
}


fn main() {
    let mut rl = Editor::<()>::new();
    let mut world = World::new();

    let white_room = Room::new("", HashMap::new());

    let mut sm = StateMachine::new(white_room);

    loop {
        println!("Gimmie a val!");
        match rl.readline(">> ") {
            Err(_) => {
                println!("Bye!");
                break;
            }
            Ok(val) => {
                if ["exit", "quit"].contains(&val.as_str()) {
                    println!("Bye!");
                    break;
                }
                println!("val: {}", val);
            }
        }
    }
}
