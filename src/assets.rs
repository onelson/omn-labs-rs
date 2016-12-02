use radiant_rs::{Renderer, Sprite};
use std::sync::Arc;
use std::collections::HashMap;

pub mod ids {
    pub const LOGO: u8 = 1;
}

#[derive(Clone)]
pub struct AssetManager<'a> {
    renderer: &'a Renderer<'a>,
    sprites: HashMap<u8, Arc<Sprite<'a>>>
}


impl<'a> AssetManager<'a> {
    pub fn new(renderer: &Renderer<'a>) -> AssetManager<'a> {
        AssetManager { renderer: renderer, sprites: HashMap::new() }
    }

    fn load(&self, id: u8) -> Arc<Sprite> {
        let fp = match id {  // FIXME: need a way to map ids to file paths that does not have a long search time - does this?
            ids::LOGO => r"assets/rust.png"
        };
        Arc::new(Sprite::from_file(&self.renderer.context(), fp))
    }

    pub fn get_sprite(&mut self, id: u8) -> &Arc<Sprite> {
        if !self.sprites.contains_key(&id) {
            self.sprites.insert(id, self.load(id));
        }
        self.sprites.get(&id).unwrap()
    }
}
