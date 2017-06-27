
extern crate time;
extern crate specs;
extern crate ggez;
extern crate rand;

extern crate omn_labs;

use omn_labs::components;
use omn_labs::systems;
use omn_labs::assets;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::timer;
use specs::{Join, WriteStorage, DispatcherBuilder, World, Dispatcher, Fetch};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use assets::AssetBundle;

use systems::DrawCommand;

pub struct DeltaTime(f32);

#[derive(Clone)]
pub struct Spinner {
    pub factor: f32,
}

impl<'a> specs::System<'a> for Spinner {
    type SystemData = (WriteStorage<'a, components::Body>, Fetch<'a, DeltaTime>);
    fn run(&mut self, data: Self::SystemData) {

        let (mut body, delta) = data;
        let dt = delta.0;
        // update entities
        for b in (&mut body).join() {
            b.rotation += dt * self.factor * rand::random::<f32>();
        }
    }
}



pub struct Game<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
}


impl<'a, 'b> Game<'a, 'b> {
    pub fn new(render_tx: Sender<DrawCommand>) -> Self {
        // The world is in charge of component storage, and as such contains all the game state.
        let mut world = World::new();
        world.register::<components::Sprited>();
        world.register::<components::Body>();
        world.add_resource(DeltaTime(0.));

        // entities are created by combining various components via the world
        world.create_entity()
            .with(components::Sprited { path: "/rust_128x128x1.png".to_string() })
            .with(components::Body {
                x: 150.,
                y: 150.,
                scale_x: 1.,
                scale_y: 1.,
                rotation: 0.,
            })
            .build();

        let dispatcher = DispatcherBuilder::new().add(
            Spinner { factor: 25. },
            "spinner",
            &[]
        ).add(
            systems::Renderer { tx: render_tx.clone() },
            "renderer",
            &[]
        ).build();

        Game {
            dispatcher: dispatcher,
            world: world
        }
    }

    pub fn tick(&mut self, dt: f32) -> () {

        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt);
        }

        self.dispatcher.dispatch(&mut self.world.res);

    }
}


struct MainState<'a, 'b> {
    ecs: Game<'a, 'b>,
    render_rx: Receiver<DrawCommand>,
    assets: AssetBundle,
}


impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        ctx.print_resource_stats();

        let (tx, rx) = channel::<DrawCommand>();

        let s = MainState {
            render_rx: rx,
            ecs: Game::new(tx),
            assets: AssetBundle::new(ctx, &vec!["/rust_128x128x1.png"]),
        };
        Ok(s)
    }
}


impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        let delta_secs = _dt.subsec_nanos() as f32 / 1e9;
        self.ecs.tick(delta_secs);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for cmd in self.render_rx.try_iter() {
            match cmd {
                DrawCommand::DrawTransformed { path, x, y, rot , .. } => {
                    let image = self.assets.get_image(ctx, path.as_ref());
                    graphics::draw(ctx, image, graphics::Point::new(x, y), rot)?;
                }
                DrawCommand::Flush => {}
            }
        }

        graphics::present(ctx);
        println!("Approx FPS: {}", timer::get_fps(ctx));
        Ok(())
    }
}


pub fn main() {

    let mut conf = conf::Conf::new();
    conf.window_height = 300;
    conf.window_width = 300;
    conf.window_title = "Omn Labs RS".to_string();

    println!("Starting with default config: {:#?}", conf);

    let ctx = &mut Context::load_from_conf("Omn Labs", "omnlabs", conf).unwrap();

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
