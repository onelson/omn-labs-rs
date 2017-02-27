
extern crate time;
extern crate specs;
extern crate ggez;
extern crate rand;

mod components;
mod game;
mod systems;

use ggez::audio;
use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, Image};
use ggez::timer;
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use systems::DrawCommand;

struct MainState {
    ecs: game::Game,
    render_rx: Receiver<DrawCommand>
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        ctx.print_resource_stats();

        let (tx, rx) = channel::<DrawCommand>();

        let s = MainState {
            render_rx: rx,
            ecs: game::Game::new(tx)
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.ecs.tick();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for cmd in self.render_rx.try_iter() {
            match cmd {
                DrawCommand::DrawTransformed {path, frame, x, y, rot, sx, sy} => {

                    // FIXME: use asset manager instead of reading from disk each tick
                    let mut image = Image::new(ctx, path).unwrap();
                    let bbox = image.rect();
                    graphics::draw(ctx,
                                   &mut image,
                                   bbox,
                                   graphics::Point::new(x, y),
                                   rot)?;
                },
                DrawCommand::Flush => {}
            }
        }

        graphics::present(ctx);
        println!("Approx FPS: {}", timer::get_fps(ctx));
        timer::sleep_until_next_frame(ctx, 60);
        Ok(())
    }
}


pub fn main() {

    let mut conf = conf::Conf::new();
    conf.window_height = 300;
    conf.window_width = 300;
    conf.window_title = "Omn Labs RS".to_string();

    println!("Starting with default config: {:#?}", conf);

    let ctx = &mut Context::load_from_conf("Omn Labs", conf).unwrap();

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}