
use ggez::Context;
use ggez::graphics;
use std::collections::HashMap;


pub struct AssetManager {
    sprites: HashMap<String, graphics::Image>
}


impl AssetManager {
    pub fn new() -> Self {
        AssetManager { sprites: HashMap::new() }
    }

    pub fn get_sprite(&mut self, ctx: &mut Context, path: &str) -> &graphics::Image {
        if !self.sprites.contains_key(path) {

            let sprite = graphics::Image::new(ctx, path)?;

            self.sprites.insert(path.to_string(), sprite);
        }
        self.sprites.get(path).unwrap()
    }
}
