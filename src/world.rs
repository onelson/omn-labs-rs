extern crate specs;

use sys;


#[derive(Default, Clone)]
pub struct Body {
    x: f64,
    y: f64,
    rotation: f64,
}

impl specs::Component for Body {
    type Storage = specs::VecStorage<Body>;
}

#[derive(Clone)]
pub struct Sprited {
    sprite: sys::Sprite
}

impl specs::Component for Sprited {
    type Storage = specs::VecStorage<Sprited>;
}
