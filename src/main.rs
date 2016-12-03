
extern crate time;
extern crate specs;
extern crate rand;
extern crate uuid;
extern crate radiant_rs;

use radiant_rs::{DisplayInfo, Display, Renderer, Layer, Color, utils};

mod game;
mod sys;
mod world;
mod assets;

use assets::AssetManager;

fn main() {
    let (width, height) = (300, 300);
    let display = Display::new(DisplayInfo { width: width, height: height, vsync: true, ..DisplayInfo::default() });
    let renderer = Renderer::new(&display);


    let assets = AssetManager::new(&renderer);
    let game = game::Game::new(&assets);

    std::thread::spawn(|| {
        let mut game = game;
        while game.tick() {}
    });

    utils::renderloop(|state| {

        // clear the layer (layers can be drawn multiple times, e.g. a static UI might not need to be updated each frame)
        game.layer.clear();

        // draw the layer
        renderer.clear_target(Color::black());
        renderer.draw_layer(&game.layer);
        renderer.swap_target();

        // poll for new events on the display, exit loop if the window was closed
        !display.poll_events().was_closed()
    });
}
