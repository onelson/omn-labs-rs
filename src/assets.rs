
use ggez::Context;
use ggez::graphics::Image;
use std::collections::HashMap;

/// `AssetBundle` acts as a file loader and cache. Currently supports Image formats supported by
/// `ggez::graphics::Image`.
///
/// # Examples
// Note that this will not run because the resource system will not be able to locate the image
// file during the test run.
/// ```no_run
///
/// extern crate ggez;
/// extern crate omn_labs;
///
/// use ggez::conf;
/// use ggez::Context;
/// use omn_labs::assets::AssetBundle;
///
/// fn main() {
///     let mut conf = conf::Conf::new();
///     conf.window_height = 300;
///     conf.window_width = 300;
///     conf.window_title = "Omn Labs RS".to_string();
///
///     let ctx = &mut Context::load_from_conf("Omn Labs", "omnlabs", conf).unwrap();
///
///     let assets = AssetBundle::new(ctx, &vec!["/rust_128x128x1.png"]);
///
///     // Paths are relative to the application's resource root.
///     // The first time an image is requested, it is cached.
///     let rust_logo1 = assets.get_image("/rust_128x128x1.png");
///
///     // Subsequent requests for the same path should return a reference to the cached image.
///     let rust_logo2 = assets.get_image("/rust_128x128x1.png");
/// }
/// ```
pub struct AssetBundle {
    images: HashMap<String, Image>,
}


impl AssetBundle {
    fn load_image(ctx: &mut Context, path: &str) -> Image {
        Image::new(ctx, path).unwrap()
    }

    pub fn new(ctx: &mut Context, image_sources: &[&str]) -> Self {
        let mut image_cache = HashMap::new();
        for src in image_sources {
            image_cache.insert(src.to_string(), Self::load_image(ctx, src));
        }
        AssetBundle {
            images: image_cache,
        }
    }

    pub fn get_image(&self, path: &str) -> &Image {
        &self.images[path]
    }
}
