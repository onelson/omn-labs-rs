
extern crate time;
extern crate specs;
extern crate rand;
extern crate radiant_rs;

mod assets;
mod components;
mod game;
mod systems;

use std::sync::mpsc::channel;
use std::sync::Arc;
use radiant_rs::{DisplayInfo, Display, Renderer, Layer, Color, utils};
use systems::DrawCommand::{self, Flush, DrawTransformed};

fn main() {

    let (width, height) = (300, 300);

    let display = Display::new(DisplayInfo {
        width: width,
        height: height,
        vsync: true,
        ..DisplayInfo::default()
    });

    let renderer = Renderer::new(&display);
    let (tx, rx) = channel::<DrawCommand>();
    let mut asset_manager = assets::AssetManager::new(&renderer);
    let main_layer = Arc::new(Layer::new(width, height));
    let mut game = game::Game::new(tx.clone());

    std::thread::spawn(move || {
        while game.tick() {
            let _ = tx.send(Flush);
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    });

    utils::renderloop(|state| {
        match rx.recv().unwrap() {
            DrawTransformed {id, frame, x, y, color, rot, sx, sy} => {
                let sprite = asset_manager.get_sprite(&id);
                sprite.draw_transformed(
                    &main_layer, frame, x, y, color, rot, sx, sy
                );
            },

            Flush => {
                renderer.clear_target(Color::white());
                renderer.draw_layer(&main_layer);
                renderer.swap_target();
                main_layer.clear();
            }
        }

        !display.poll_events().was_closed()
    });

}
