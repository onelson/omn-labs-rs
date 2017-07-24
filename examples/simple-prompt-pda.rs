extern crate rustyline;
extern crate omn_labs;

use omn_labs::state::{State, StateMachine};
use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    let mut rl = Editor::<()>::new();
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
