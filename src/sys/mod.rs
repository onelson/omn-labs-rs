extern crate piston_window;
extern crate sprite;

pub mod render;
pub type Delta = f32;
pub type Sprite = sprite::Sprite<piston_window::ImageSize>;
pub type Scene = sprite::Scene<piston_window::ImageSize>;
