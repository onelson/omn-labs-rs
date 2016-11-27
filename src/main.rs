extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate time;
extern crate specs;

use piston_window::*;
use sprite::*;

mod game;
mod sys;
mod world;

fn main() {
    let (width, height) = (300, 300);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
    WindowSettings::new("OmnLabs", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let id;
    let mut scene = Scene::new();

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });

    }

    let game = game::Game::new(&window, &scene);
    std::thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });
}