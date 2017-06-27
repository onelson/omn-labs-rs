use specs;
use specs::{Join, ReadStorage, WriteStorage, System};
use std::sync::mpsc::Sender;
use components;
use Delta;

pub enum DrawCommand {
    DrawTransformed {
        path: String,
        frame: u32,
        x: f32,
        y: f32,
        rot: f32,
        sx: f32,
        sy: f32,
    },
    Flush,
}

#[derive(Clone)]
pub struct Renderer {
    pub tx: Sender<DrawCommand>,
}


impl<'a> System<'a> for Renderer {
    type SystemData = (ReadStorage<'a, components::Body>, ReadStorage<'a, components::Sprited>);
    fn run(&mut self, data: Self::SystemData) {

        let (body, sprited) = data;
        // update entities
        for (b, s) in (&body, &sprited).join() {
            let frame_id = 0;
            self.tx
                .send(DrawCommand::DrawTransformed {
                    path: s.path.to_string(),
                    frame: frame_id,
                    x: b.x,
                    y: b.y,
                    rot: b.rotation,
                    sx: b.scale_x,
                    sy: b.scale_y,
                })
                .unwrap();
        }
    }
}
