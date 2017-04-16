use specs;
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


impl specs::System<Delta> for Renderer {
    fn run(&mut self, arg: specs::RunArg, _: Delta) {
        use specs::Join;
        let (body, sprited) =
            arg.fetch(|w| (w.read::<components::Body>(), w.read::<components::Sprited>()));

        // update entities
        for (b, s) in (&body, &sprited).iter() {
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
