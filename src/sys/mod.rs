use sprite;
use piston_window;

pub mod spinner;
pub mod render;
pub type Delta = f64;
pub type Sprite = sprite::Sprite<piston_window::ImageSize>;
pub type Scene = sprite::Scene<piston_window::ImageSize>;
