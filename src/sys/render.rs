use specs;
use world as w;
use rand;
use sys;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use radiant_rs::{Color, Layer};
use assets::AssetManager;


pub enum DrawCommand {
    DrawTransformed {
        id: u8,
        layer: Arc<Layer>,
        frame: u32,
        color: Color,
        x: f32,
        y: f32,
        rot: f32,
        sx: f32,
        sy: f32
    },
    Clear,
    Flush
}

#[derive(Clone)]
pub struct System {
    pub layer: Arc<Layer>,
    pub tx: Sender<DrawCommand>
}


impl specs::System<super::Delta> for System
{
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;
        let (body, sprited) = arg.fetch(|w| {
            (w.read::<w::Body>(), w.read::<w::Sprited>())
        });

        // update entities
        for (b, s) in (&body, &sprited).iter() {
            let frame_id = 0;
            self.tx.send(DrawCommand::DrawTransformed {
                id: s.id,
                layer: self.layer.clone(),
                frame: frame_id,
                color: Color::white(),
                x: b.x,
                y: b.y,
                rot: b.rotation,
                sx: b.scale_x,
                sy: b.scale_y
            }).unwrap();
        }
    }
}
