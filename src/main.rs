extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate time;
extern crate specs;
extern crate graphics;

use std::rc::Rc;

use piston_window::*;
use sprite::*;

mod game;
mod sys;
mod world;

fn create_logo_sprite(window: PistonWindow, width: u32, height: u32) -> Sprite<ImageSize> {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let tex = Rc::new(Texture::from_path(
        &mut window.factory,
        assets.join("rust.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap());
    let sprite = Sprite::from_texture(tex.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);
    sprite

}

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

    let mut sprite = create_logo_sprite(window, width, height);
    scene.add_child(sprite);

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });

    }

    let game = game::Game::new(&sprite);
    std::thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });
}