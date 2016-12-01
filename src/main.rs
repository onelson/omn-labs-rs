extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate time;
extern crate specs;
extern crate rand;
extern crate uuid;

use std::rc::Rc;
use piston_window::*;

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

    let mut sprite = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let tex = Rc::new(Texture::from_path(
            &mut window.factory,
            assets.join("rust.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
        sprite::Sprite::from_texture(tex.clone())
    };

    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    let mut game = game::Game::new(sprite);

    while let Some(e) = window.next() {
        game.tick();
        game.scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            game.scene.draw(c.transform, g);
        });
    }

}