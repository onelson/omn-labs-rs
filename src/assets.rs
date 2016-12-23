
use radiant_rs::{RenderContext, Renderer, Sprite};
use std::collections::HashMap;


pub mod ids {
    pub const RUST_LOGO: u8 = 1;
}


#[derive(Clone)]
pub struct AssetManager<'a> {
    context: RenderContext<'a>,
    sprites: HashMap<u8, Sprite<'a>>
}


impl<'a> AssetManager<'a> {
    pub fn new(renderer: &'a Renderer<'a>) -> Self {
        AssetManager { context: renderer.context(), sprites: HashMap::new() }
    }

    fn load(&self, id: &u8) -> Sprite<'a> {
        let fp = match *id {
            ids::RUST_LOGO => r"assets/rust_128x128x1.png",
            _ => unreachable!()
        };
        Sprite::from_file(&self.context, fp)
    }

    pub fn get_sprite(&mut self, id: &u8) -> &Sprite<'a> {
        if !self.sprites.contains_key(id) {
            let sprite = self.load(id);
            self.sprites.insert(*id, sprite);
        }
        self.sprites.get(&id).unwrap()
    }
}
