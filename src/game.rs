extern crate sprite;
extern crate time;
extern crate specs;
extern crate piston_window;

use sprite::*;
use piston_window::*;

use sys;
use world;


pub struct Game<'a> {
    planner: specs::Planner<sys::Delta>,
    last_time: u64,
    last_update: u64,
    frame_count: f64,
    logo: &'a Sprite<ImageSize>,
}


impl Game<'a> {
    pub fn new(logo: &'a Sprite<ImageSize>)
    {
        let mut w = specs::World::new();
        w.register::<world::Sprited>();
        w.register::<world::Body>();

        // prepare systems
        let mut render_sys = sys::render::System::new();
        // prepare entities



        let mut plan = specs::Planner::new(w, 2);
        plan.add_system(render_sys, "render", 10);

        Game {
            last_update: 0,
            planner: plan,
            last_time: time::precise_time_ns(),
            logo: logo,
            frame_count: 0
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = time::precise_time_ns();
        let delta = (new_time - self.last_time) as sys::Delta / 1e9;
        self.last_time = new_time;

        self.last_update += delta;
        self.frame_count += 1;

        if self.last_update >= 0.5 {
            let fps = self.frame_count / self.last_update;
            self.last_update = 0.0;
            self.frame_count = 0.0;
            print!("{:?} fps", fps);
        }

        self.planner.dispatch(delta);
        self.planner.word.is_alive(self.player)
    }
}

