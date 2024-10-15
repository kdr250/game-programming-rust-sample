use std::{cell::RefCell, rc::Rc};

use crate::{component::Component, math::Vector2, Game};

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Active,
    Paused,
    Dead,
}

pub trait Actor {
    /// Update function called from Game (not overridable)
    fn update(&mut self, delta_time: f32);

    /// Updates all the components attached to the actor (not overridable)
    fn update_component(&mut self, delta_time: f32);

    /// Any actor-specific update code (overridable)
    fn update_actor(&mut self, delta_time: f32);

    /// Getters/setters
    fn get_position(&self) -> &Vector2;

    fn set_position(&mut self, position: Vector2);

    fn get_scale(&self) -> f32;

    fn set_scale(&mut self, scale: f32);

    fn get_rotation(&self) -> f32;

    fn set_rotation(&mut self, rotation: f32);

    fn get_state(&self) -> &State;

    fn set_state(&mut self, state: State);

    fn get_game(&self) -> &Rc<RefCell<Game>>;

    /// Add/remove components
    fn add_component(&mut self, component: Rc<RefCell<dyn Component>>);

    fn remove_component(&mut self, component: Rc<RefCell<dyn Component>>);

    fn get_cocmponents(&self) -> &Vec<Rc<RefCell<dyn Component>>>;
}

#[cfg(test)]
pub mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{component::Component, math::Vector2, Game};

    use super::{Actor, State};

    pub struct TestActor {
        state: State,
        position: Vector2,
        scale: f32,
        rotation: f32,
        components: Vec<Rc<RefCell<dyn Component>>>,
        game: Option<Rc<RefCell<Game>>>,
    }

    impl TestActor {
        pub fn new() -> Self {
            Self {
                state: State::Active,
                position: Vector2::ZERO,
                scale: 1.0,
                rotation: 0.0,
                components: vec![],
                game: None,
            }
        }
    }

    impl Actor for TestActor {
        fn update(&mut self, delta_time: f32) {
            if self.state == State::Active {
                self.update_component(delta_time);
                self.update_actor(delta_time);
            }
        }

        fn update_component(&mut self, delta_time: f32) {
            for component in &self.components {
                component.borrow_mut().update(delta_time);
            }
        }

        fn update_actor(&mut self, delta_time: f32) {}

        fn get_position(&self) -> &Vector2 {
            &self.position
        }

        fn set_position(&mut self, position: Vector2) {
            self.position = position;
        }

        fn get_scale(&self) -> f32 {
            self.scale
        }

        fn set_scale(&mut self, scale: f32) {
            self.scale = scale;
        }

        fn get_rotation(&self) -> f32 {
            self.rotation
        }

        fn set_rotation(&mut self, rotation: f32) {
            self.rotation = rotation;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn set_state(&mut self, state: State) {
            self.state = state;
        }

        fn get_game(&self) -> &Rc<RefCell<Game>> {
            todo!();
        }

        fn add_component(&mut self, component: Rc<RefCell<dyn Component>>) {
            self.components.push(component);
        }

        fn remove_component(&mut self, component: Rc<RefCell<dyn Component>>) {
            self.components
                .retain(|c| c.borrow().id() != component.borrow().id());
        }

        fn get_cocmponents(&self) -> &Vec<Rc<RefCell<dyn Component>>> {
            &self.components
        }
    }
}
