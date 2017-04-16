
extern crate time;
extern crate specs;
extern crate ggez;

extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub mod components;
pub mod systems;
pub mod assets;
pub mod sprites;

pub type Delta = f32;