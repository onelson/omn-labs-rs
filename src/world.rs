use specs;

#[derive(Default, Clone, Debug)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

impl specs::Component for Body {
    type Storage = specs::VecStorage<Body>;
}
