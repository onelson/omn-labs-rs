use time;
use specs;

use sys;
use world;

use std::sync::Arc;
use radiant_rs::{Layer, Renderer, Sprite};
use assets::{AssetManager, SpriteSrc};

pub struct Game {
    pub world: specs::World,
    pub planner: specs::Planner<sys::Delta>,
    pub layer: Layer,
    assets: AssetManager,
    last_time: u64,
    last_update: f64,
    frame_count: f64,
}


impl Game {
    pub fn new(renderer: &Renderer) -> Game
    {

        let assets = AssetManager::new(&renderer);

        let logo_id = assets.load_sprite(r"assets/rust.png");


        let (width, height) = (300, 300);
        let layer = Layer::new(width, height);

        let w = specs::World::new();
        w.register::<world::Sprited>();
        w.register::<world::Body>();

        // prepare systems
        let spinner_sys = sys::spinner::System::new();
        let render_sys = sys::render::System { layer: Arc::new(&layer), assets: assets };

        // prepare entities

        w.create_now()
            .with(world::Sprited { id: logo_id })
            .with(world::Body::default())
            .build();

        let mut plan = specs::Planner::new(w, 2);
        plan.add_system(spinner_sys, "spinner", 10);
        plan.add_system(render_sys, "render_layer", 20);

        Game {
            last_update: 0.0,
            planner: plan,
            layer: layer,
            assets,
            world: w,
            last_time: time::precise_time_ns(),
            frame_count: 0.0
        }
    }

    pub fn tick(&mut self) -> bool {

        let new_time = time::precise_time_ns();
        let delta = (new_time - self.last_time) as sys::Delta / 1e9;
        self.last_time = new_time;

        self.last_update += delta;
        self.frame_count += 1.0;

        if self.last_update >= 0.5 {
            let fps = self.frame_count / self.last_update;
            self.last_update = 0.0;
            self.frame_count = 0.0;
            println!("{:.3} fps", fps);
        }

        self.planner.dispatch(delta);

        //        self.planner.word.is_alive(self.logo)
        true
    }
}

