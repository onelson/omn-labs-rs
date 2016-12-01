
extern crate time;
extern crate specs;
extern crate rand;
extern crate uuid;
extern crate radiant_rs;

use radiant_rs::{DisplayInfo, Display, Renderer, Layer, Sprite, Font, FontInfo, Color, blendmodes, utils};

mod game;
mod sys;
mod world;

fn main() {
    let (width, height) = (300, 300);
    let display = Display::new(DisplayInfo { width: width, height: height, vsync: true, ..DisplayInfo::default() });
    let renderer = Renderer::new(&display);

    let sprite = Sprite::from_file(&renderer.context(), r"assets/rust.png");

//    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);



    let game = game::Game::new(&layer);

    std::thread::spawn(|| {
        let mut game = game;
        while game.tick() {}
    });

    utils::renderloop(|state| {

        // clear the layer (layers can be drawn multiple times, e.g. a static UI might not need to be updated each frame)
        game.layer.clear();

        // colorize the whole layer
//        let rainbow = Color::from_hsl((state.elapsed_f32/5.0).fract(), 1.0, 0.5, 1.0);
//        layer.set_color(rainbow);

        // rotate the layer as a whole (by contrast, layer.model_matrix() would rotate the individual sprites)
//        game.layer.view_matrix().rotate_at((320.0, 200.0), -state.delta_f32);

        // write some text
//        font.write(&layer, &format!("It works. {} FPS", state.fps), 260.0, 140.0);

        // draw a sprite (going though the spritesheet frames at 30 fps)
//        let frame_id = (state.elapsed_f32 * 30.0) as u32;
        let frame_id = 0;
        sprite.draw_transformed(&game.layer, frame_id, 320.0, 200.0, Color::white(), rotation, 1, 1);

        // draw the layer
        renderer.clear_target(Color::black());
        renderer.draw_layer(&game.layer);
        renderer.swap_target();

        // poll for new events on the display, exit loop if the window was closed
        !display.poll_events().was_closed()
    });

}