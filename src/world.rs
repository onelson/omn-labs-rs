extern crate specs;
extern crate sprite;
extern crate graphics;

use graphics::ImageSize;


#[derive(Default)]
pub struct Body {
    x: f64,
    y: f64,
    rotation: f64,
}

impl specs::Component for Body {
    type Storage = specs::VecStorage<Body>;
}

#[derive(Default)]
pub struct Sprited {
    sprite: sprite::Sprite<ImageSize>
}

impl specs::Component for Sprited {
    type Storage = specs::VecStorage<Sprited>;
}
