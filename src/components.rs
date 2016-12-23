use specs;


#[derive(Clone, Debug)]
pub struct Body {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
}

impl specs::Component for Body {
    type Storage = specs::VecStorage<Body>;
}

#[derive(Clone, Debug)]
pub struct Sprited {
    pub id: u8
}

impl specs::Component for Sprited {
    type Storage = specs::VecStorage<Sprited>;
}
