
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
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use assets::AssetManager;

use systems::DrawCommand;

#[derive(Clone)]
pub struct Spinner {
    pub factor: f32,
}

impl specs::System<omn_labs::Delta> for Spinner {
    fn run(&mut self, arg: specs::RunArg, dt: omn_labs::Delta) {
        use specs::Join;

        let mut body = arg.fetch(|w| w.write::<components::Body>());

        // update entities
        for b in (&mut body).iter() {
            b.rotation += dt * self.factor * rand::random::<f32>();
        }
    }
}



pub struct Game {
    pub planner: specs::Planner<omn_labs::Delta>,
}


impl Game {
    pub fn new(render_tx: Sender<DrawCommand>) -> Game {
        // The world is in charge of component storage, and as such contains all the game state.
        let mut world = specs::World::new();
        world.register::<components::Sprited>();
        world.register::<components::Body>();

        let spinner_sys = Spinner { factor: 25. };
        let render_sys = systems::Renderer { tx: render_tx.clone() };

        // entities are created by combining various components via the world
        world.create_now()
            .with(components::Sprited { path: "rust_128x128x1.png".to_string() })
            .with(components::Body {
                x: 150.,
                y: 150.,
                scale_x: 1.,
                scale_y: 1.,
                rotation: 0.,
            })
            .build();

        // systems are registered with a planner, which manages their execution
        let mut plan = specs::Planner::new(world, 2);
        plan.add_system(spinner_sys, "spinner", 10);
        plan.add_system(render_sys, "render_layer", 20);

        Game { planner: plan }
    }

    pub fn tick(&mut self, dt: omn_labs::Delta) -> bool {

        // dispatch() tells the planner to run the registered systems in a
        // thread pool.
        self.planner.dispatch(dt);

        // the wait() is like a thread.join(), and will block until the systems
        // have completed their work.
        self.planner.wait();
        true
    }
}


struct MainState {
    ecs: Game,
    render_rx: Receiver<DrawCommand>,
    assets: AssetManager,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        ctx.print_resource_stats();

        let (tx, rx) = channel::<DrawCommand>();

        let s = MainState {
            render_rx: rx,
            ecs: Game::new(tx),
            assets: AssetManager::new(),
        };
        Ok(s)
    }
}


impl event::EventHandler for MainState {
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
                    let image = self.assets.get_sprite(ctx, path.as_ref());
                    graphics::draw(ctx, image, graphics::Point::new(x, y), rot)?;
                }
                DrawCommand::Flush => {}
            }
        }

        graphics::present(ctx);
        println!("Approx FPS: {}", timer::get_fps(ctx));
        //        timer::sleep_until_next_frame(ctx, 60);
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
