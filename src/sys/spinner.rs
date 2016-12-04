//use std::sync::{mpsc, Arc};
use specs;
use world as w;
use rand;

#[derive(Clone)]
pub struct System {
    factor: f32
}

impl System {
    pub fn new() -> System {
        System { factor: 2.5 }
    }
}

impl specs::System<super::Delta> for System
{
    fn run(&mut self, arg: specs::RunArg, delta: super::Delta) {
        use specs::Join;

        let mut body = arg.fetch(|w| {
            w.write::<w::Body>()
        });

        // update entities
        for b in (&mut body).iter() {
            b.rotation += self.factor % rand::random::<f32>();
        }
    }
}
