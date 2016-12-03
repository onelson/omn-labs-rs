use specs;
use world as w;
use rand;
use sys;
use std::sync::Arc;
use radiant_rs::{Color, Layer};
use assets::AssetManager;


#[derive(Clone)]
pub struct System<'a, 'b> {
    pub layer: &'a Layer,
    pub assets: &'b AssetManager<'b>
}


impl<'a, 'b> specs::System<super::Delta> for System<'a, 'b>
{
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;
        let (body, sprited) = arg.fetch(|w| {
            (w.read::<w::Body>(), w.read::<w::Sprited>())
        });

        // update entities
        for (b, s) in (&body, &sprited).iter() {
            let frame_id = 0;
            let sprite = self.assets.get_sprite(s.id);
            sprite.draw_transformed(&self.layer, frame_id, b.x, b.y, Color::white(), b.rotation, b.scale_x, b.scale_y);
        }
    }
}
