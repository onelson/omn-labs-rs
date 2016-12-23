use time;
use specs;

use systems;
use components;

use std::sync::mpsc::Sender;
use assets;

pub type Delta = f32;


pub struct Game {
    pub planner: specs::Planner<Delta>,
    last_time: u64
}


impl Game {
    pub fn new(render_tx: Sender<systems::DrawCommand>) -> Game
    {
        // The world is in charge of component storage, and as such contains all the game state.
        let mut world = specs::World::new();
        world.register::<components::Sprited>();
        world.register::<components::Body>();

        let spinner_sys = systems::Spinner { factor: 25. };
        let render_sys = systems::Renderer { tx: render_tx.clone() };

        // entities are created by combining various components via the world
        world.create_now()
            .with(components::Sprited { id: assets::ids::RUST_LOGO })
            .with(components::Body { x: 150., y: 150., scale_x: 1., scale_y: 1., rotation: 0. })
            .build();

        // systems are registered with a planner, which manages their execution
        let mut plan = specs::Planner::new(world, 2);
        plan.add_system(spinner_sys, "spinner", 10);
        plan.add_system(render_sys, "render_layer", 20);

        Game {
            planner: plan,
            last_time: time::precise_time_ns()
        }
    }

    pub fn tick(&mut self) -> bool {

        let new_time = time::precise_time_ns();
        let delta = (new_time - self.last_time) as Delta / 1e9;
        self.last_time = new_time;

        // dispatch() tells the planner to run the registered systems in a
        // thread pool.
        self.planner.dispatch(delta);

        // the wait() is like a thread.join(), and will block until the systems
        // have completed their work.
        self.planner.wait();
        true
    }
}

