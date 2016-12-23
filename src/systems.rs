use specs;
use rand;
use std::sync::mpsc::Sender;
use radiant_rs::{Color};
use game;
use components;


#[derive(Clone)]
pub struct Spinner {
    pub factor: f32
}


impl specs::System<game::Delta> for Spinner
{
    fn run(&mut self, arg: specs::RunArg, dt: game::Delta) {
        use specs::Join;

        let mut body = arg.fetch(|w| {
            w.write::<components::Body>()
        });

        // update entities
        for b in (&mut body).iter() {
            b.rotation += dt * self.factor * rand::random::<f32>();
        }
    }
}


pub enum DrawCommand {
    DrawTransformed {
        id: u8,
        frame: u32,
        color: Color,
        x: f32,
        y: f32,
        rot: f32,
        sx: f32,
        sy: f32
    },
    Flush
}

#[derive(Clone)]
pub struct Renderer {
    pub tx: Sender<DrawCommand>
}

impl specs::System<game::Delta> for Renderer
{
    fn run(&mut self, arg: specs::RunArg, _: game::Delta) {
        use specs::Join;
        let (body, sprited) = arg.fetch(|w| {
            (w.read::<components::Body>(), w.read::<components::Sprited>())
        });

        // update entities
        for (b, s) in (&body, &sprited).iter() {
            let frame_id = 0;
            self.tx.send(DrawCommand::DrawTransformed {
                id: s.id,
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
