//use std::sync::{mpsc, Arc};
use specs;
use world as w;

pub struct System;


impl System
{
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;
        let (graphic, body) = arg.fetch(|w| {
            (w.read::<w::Sprited>(), w.read::<w::Body>())
        });

        // update entities
        for (g, b) in (&graphic, &body).iter() {

        }
    }
}