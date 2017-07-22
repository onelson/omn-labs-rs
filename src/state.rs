use ggez::{conf, Context};
use specs::World;
use assets::AssetBundle;

pub struct WindowEvent;


/// Types of state transitions.
pub enum Trans {
    /// Continue as normal.
    None,
    /// Remove the active state and resume the next state on the stack or stop
    /// if there are none.
    Pop,
    /// Pause the active state and push a new state onto the stack.
    Push(Box<State>),
    /// Remove the current state on the stack and insert a different one.
    Switch(Box<State>),
    /// Stop and remove all states and shut down the engine.
    Quit,
}

/// A trait which defines game states that can be used by the state machine.
pub trait State {
    /// Executed when the game state begins.
    fn on_start(&mut self, _world: &mut World, _assets: &mut AssetBundle, _ctx: &mut Context) {}

    /// Executed when the game state exits.
    fn on_stop(&mut self, _world: &mut World, _assets: &mut AssetBundle, _ctx: &mut Context) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_pause(&mut self, _world: &mut World, _assets: &mut AssetBundle, _ctx: &mut Context) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, _world: &mut World, _assets: &mut AssetBundle, _ctx: &mut Context) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(&mut self,
                     _events: &[WindowEvent],
                     _world: &mut World,
                     _assets: &mut AssetBundle,
                     _ctx: &mut Context)
                     -> Trans {
        Trans::None
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default).
    fn fixed_update(&mut self,
                    _world: &mut World,
                    _assets: &mut AssetBundle,
                    _ctx: &mut Context)
                    -> Trans {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow.
    fn update(&mut self,
              _world: &mut World,
              _assets: &mut AssetBundle,
              _ctx: &mut Context)
              -> Trans {
        Trans::None
    }
}

/// A simple stack-based state machine (pushdown automaton).
pub struct StateMachine {
    running: bool,
    state_stack: Vec<Box<State>>,
}

impl StateMachine {
    /// Creates a new state machine with the given initial state.
    pub fn new<T>(initial_state: T) -> StateMachine
        where T: State + 'static
    {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
        }
    }

    /// Checks whether the state machine is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Initializes the state machine.
    ///
    /// # Panics
    /// Panics if no states are present in the stack.
    pub fn start(&mut self, world: &mut World, assets: &mut AssetBundle, ctx: &mut Context) {
        if !self.running {
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(world, assets, ctx);
            self.running = true;
        }
    }

    /// Passes a vector of events to the active state to handle.
    pub fn handle_events(&mut self,
                         events: &[WindowEvent],
                         world: &mut World,
                         assets: &mut AssetBundle,
                         ctx: &mut Context) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.handle_events(events, world, assets, ctx),
                None => Trans::None,
            };

            self.transition(trans, world, assets, ctx);
        }
    }

    /// Updates the currently active state at a steady, fixed interval.
    pub fn fixed_update(&mut self,
                        world: &mut World,
                        assets: &mut AssetBundle,
                        ctx: &mut Context) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.fixed_update(world, assets, ctx),
                None => Trans::None,
            };

            self.transition(trans, world, assets, ctx);
        }
    }

    /// Updates the currently active state immediately.
    pub fn update(&mut self, world: &mut World, assets: &mut AssetBundle, ctx: &mut Context) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.update(world, assets, ctx),
                None => Trans::None,
            };

            self.transition(trans, world, assets, ctx);
        }
    }

    /// Performs a state transition, if requested by either update() or
    /// fixed_update().
    fn transition(&mut self,
                  request: Trans,
                  world: &mut World,
                  assets: &mut AssetBundle,
                  ctx: &mut Context) {
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(world, assets, ctx),
                Trans::Push(state) => self.push(state, world, assets, ctx),
                Trans::Switch(state) => self.switch(state, world, assets, ctx),
                Trans::Quit => self.stop(world, assets, ctx),
            }
        }
    }

    /// Removes the current state on the stack and inserts a different one.
    fn switch(&mut self,
              state: Box<State>,
              world: &mut World,
              assets: &mut AssetBundle,
              ctx: &mut Context) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(world, assets, ctx);
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(world, assets, ctx);
        }
    }

    /// Pauses the active state and pushes a new state onto the state stack.
    fn push(&mut self,
            state: Box<State>,
            world: &mut World,
            assets: &mut AssetBundle,
            ctx: &mut Context) {
        if self.running {
            if let Some(state) = self.state_stack.last_mut() {
                state.on_pause(world, assets, ctx);
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(world, assets, ctx);
        }
    }

    /// Stops and removes the active state and un-pauses the next state on the
    /// stack (if any).
    fn pop(&mut self, world: &mut World, assets: &mut AssetBundle, ctx: &mut Context) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(world, assets, ctx);
            }

            if let Some(mut state) = self.state_stack.last_mut() {
                state.on_resume(world, assets, ctx);
            } else {
                self.running = false;
            }
        }
    }

    /// Shuts the state machine down.
    fn stop(&mut self, world: &mut World, assets: &mut AssetBundle, ctx: &mut Context) {
        if self.running {
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop(world, assets, ctx);
            }

            self.running = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct State1(u8);
    struct State2;

    impl State for State1 {
        fn update(&mut self, _: &mut World, _: &mut AssetBundle, _: &mut Context) -> Trans {
            if self.0 > 0 {
                self.0 -= 1;
                Trans::None
            } else {
                Trans::Switch(Box::new(State2))
            }
        }
    }

    impl State for State2 {
        fn update(&mut self, _: &mut World, _: &mut AssetBundle, _: &mut Context) -> Trans {
            Trans::Pop
        }
    }

    #[test]
    fn switch_pop() {

        let conf = conf::Conf::new();
        let mut ctx = Context::load_from_conf("switch pop", "omnlabs", conf).unwrap();

        let mut assets = AssetBundle::new(&mut ctx, &vec!());
        let mut world = World::new();

        let mut sm = StateMachine::new(State1(7));
        sm.start(&mut world, &mut assets, &mut ctx);

        for _ in 0..8 {
            sm.update(&mut world, &mut assets, &mut ctx);
            assert!(sm.is_running());
        }

        sm.update(&mut world, &mut assets, &mut ctx);
        assert!(!sm.is_running());

    }
}
