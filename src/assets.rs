use radiant_rs::{Renderer, Sprite};
use std::sync::Arc;
use std::collections::HashMap;


pub struct AssetManager<'a> {
    renderer: &'a Renderer<'a>,
    sprites: HashMap<&'a str, Arc<Sprite<'a>>>
}


impl<'a> AssetManager<'a> {
    pub fn new(renderer: &'a Renderer) -> AssetManager<'a> {
        AssetManager { renderer: renderer, sprites: HashMap::new() }
    }

    pub fn get_sprite(&mut self, fp: &str) -> Arc<Sprite> {
        Arc::new(Sprite::from_file(&self.renderer.context(), fp))

    }
}
