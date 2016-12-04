
extern crate time;
extern crate specs;
extern crate rand;
extern crate uuid;
extern crate radiant_rs;

#[macro_use]
extern crate lazy_static;

use radiant_rs::{DisplayInfo, Display, Renderer, Layer, Color, utils};
use std::sync::Arc;
use std::sync::mpsc::channel;

mod game;
mod sys;
mod world;
mod assets;

use assets::AssetManager;

static WIDTH: u32 = 512;
static HEIGHT: u32 = 512;

fn main() {
    let display = Display::new(
        DisplayInfo {
            width: WIDTH,
            height: HEIGHT,
            vsync: true, ..DisplayInfo::default()
        }
    );
    let renderer = Renderer::new(&display);
    let mut asset_manager = AssetManager::new(&renderer);

    let (tx, rx) = channel::<sys::render::DrawCommand>();

    let layer = Arc::new(Layer::new(WIDTH, HEIGHT));
    let mut game = game::Game::new(&layer, &tx);
    std::thread::spawn(move || {
        while game.tick() {
            game.planner.wait();
            tx.send(sys::render::DrawCommand::Flush);
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    });

    utils::renderloop(|state| {
        use sys::render::DrawCommand;
        match rx.recv().unwrap() {
            DrawCommand::DrawTransformed{id, layer, frame, x, y, color, rot, sx, sy} => {
                let sprite = asset_manager.get_sprite(&id);
                sprite.draw_transformed(
                    &layer, frame, x, y, color, rot, sx, sy
                );
            },

            DrawCommand::Flush => {
                // draw the layer
                renderer.clear_target(Color::black());
                renderer.draw_layer(&layer);
                renderer.swap_target();
                layer.clear();
            }

            _ => unreachable!()
        }

        // poll for new events on the display, exit loop if the window was closed
        !display.poll_events().was_closed()
    });
}
