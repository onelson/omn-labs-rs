//use std::sync::{mpsc, Arc};
use specs;
use world as w;
use rand;

#[derive(Clone)]
pub struct System;

impl System {
    pub fn new() -> System { System{} }
}

impl specs::System<super::Delta> for System
{
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;

        let mut body = arg.fetch(|w| {
            w.write::<w::Body>()
        });

        // update entities
        for b in (&mut body).iter() {
            b.rotation += rand::random::<f32>() * 5.0;
        }
    }
}