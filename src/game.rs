use time;
use specs;

use sys;
use world;


pub struct Game {
    pub planner: specs::Planner<sys::Delta>,
    last_time: u64,
    last_update: f64,
    frame_count: f64,
}


impl Game {
    pub fn new(mut w: specs::World) -> Game
    {
//        w.register::<world::Sprited>();
        w.register::<world::Body>();

        // prepare systems
        let spinner_sys = sys::spinner::System::new();
        // prepare entities

        w.create_now().with(world::Body::default()).build();

        let mut plan = specs::Planner::new(w, 2);
        plan.add_system(spinner_sys, "spinner", 10);

        Game {
            last_update: 0.0,
            planner: plan,
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

