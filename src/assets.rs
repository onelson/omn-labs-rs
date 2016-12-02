use radiant_rs::{Renderer, Sprite};
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;


pub struct AssetManager<'a> {
    renderer: &'a Renderer<'a>,
    sprites: HashMap<&'a str, Arc<Sprite<'a>>>
}


impl<'a> AssetManager<'a> {
    pub fn new(renderer: Arc<Renderer>) -> AssetManager {
        AssetManager { renderer: renderer, sprites: HashMap::new() }
    }

    pub fn load_sprite(&mut self, path: &str) -> Uuid {
        ???
    }

    pub fn get_sprite(&mut self, id: Uuid) -> Arc<Sprite> {
        Arc::new(Sprite::from_file(&self.renderer.context(), fp))

    }
}
