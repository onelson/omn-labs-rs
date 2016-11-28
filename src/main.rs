extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate time;
extern crate specs;
extern crate rand;

use std::rc::Rc;
use sprite::{Sprite, Scene};
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

    let mut scene = Scene::new();

    let mut sprite = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let tex = Rc::new(Texture::from_path(
            &mut window.factory,
            assets.join("rust.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
        Sprite::from_texture(tex.clone())
    };

    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);
    let sprite_id = scene.add_child(sprite);

    let w = specs::World::new();
    let mut game = game::Game::new(w);

    while let Some(e) = window.next() {
        use specs::Join;
        scene.event(&e);

        game.tick();

        let w = game.planner.mut_world();
        let reader = w.read::<world::Body>();

        for b in reader.iter() {
            scene.child_mut(sprite_id).unwrap().set_rotation(b.rotation);
        }

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });
    }

}