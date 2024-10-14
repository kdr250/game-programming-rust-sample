use std::{cell::RefCell, rc::Rc};

use crate::{component::Component, math::Vector2, Game};

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

    fn get_state(&self) -> State;

    fn set_state(&mut self, state: State);

    fn get_game(&self) -> &Rc<RefCell<Game>>;

    /// Add/remove components
    fn add_component(&mut self, component: &dyn Component);

    fn remove_component(&mut self, component: &dyn Component);
}
