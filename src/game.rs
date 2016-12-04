use time;
use specs;

use sys;
use world;

use std::sync::Arc;
use std::sync::mpsc::Sender;
use radiant_rs::{Layer, Renderer, Sprite};
use assets::{AssetManager, ids as asset_ids};


pub struct Game {
    pub planner: specs::Planner<sys::Delta>,
    last_time: u64,
    last_update: f64,
    frame_count: f64,
}


impl Game {
    pub fn new(layer: &Arc<Layer>, render_tx: &Sender<sys::render::DrawCommand>) -> Self {
        let mut game = Game {
            last_update: 0.0,
            planner: specs::Planner::new( specs::World::new(), 2 ),
            last_time: time::precise_time_ns(),
            frame_count: 0.0
        };

        game.planner.mut_world().register::<world::Sprited>();
        game.planner.mut_world().register::<world::Body>();

        // prepare systems
        let spinner_sys = sys::spinner::System::new();

        let render_sys = sys::render::System {
            layer: layer.clone(),
            tx: render_tx.clone()
        };

        // prepare entities
        for i in 0 .. 10 {
            let i = i as f32;
            game.planner.mut_world().create_now()
                .with(world::Sprited { id: asset_ids::LAUGHING_MAN })
                .with(world::Body { x: 256. + i, y: 256. + i, rotation: 0., scale_x: 2.5, scale_y: 2.5 })
                .build();
        }

        game.planner.add_system(spinner_sys, "spinner", 10);
        game.planner.add_system(render_sys, "render_layer", 20);

        return game;
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
