
use ggez::Context;
use ggez::graphics::Image;
use std::collections::HashMap;


pub struct AssetManager {
    sprites: HashMap<String, Image>,
}


impl AssetManager {
    pub fn new() -> Self {
        AssetManager { sprites: HashMap::new() }
    }

    pub fn get_sprite(&mut self, ctx: &mut Context, path: &str) -> &Image {
        if !self.sprites.contains_key(path) {

            let sprite = Image::new(ctx, path).unwrap();

            self.sprites.insert(path.to_string(), sprite);
        }

        self.sprites.get(path).unwrap()
    }
}
