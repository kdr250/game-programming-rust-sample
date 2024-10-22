use std::{cell::RefCell, rc::Rc};

use crate::{
    components::component::{Component, State as ComponentState},
    math::vector2::Vector2,
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Active,
    Paused,
    Dead,
}

pub trait Actor {
    /// Update function called from Game (not overridable)
    fn update(&mut self, delta_time: f32) {
        if *self.get_state() == State::Active {
            self.update_component(delta_time);
            self.update_actor(delta_time);
        }
    }

    /// Updates all the components attached to the actor (not overridable)
    fn update_component(&mut self, delta_time: f32) {
        let mut changes: Vec<(Option<Vector2>, Option<f32>)> = vec![];
        let actor_info = (
            self.get_position().clone(),
            self.get_rotation(),
            self.get_forward(),
        );

        for component in self.get_cocmponents() {
            let change = component.borrow_mut().update(delta_time, &actor_info);
            changes.push(change);
        }

        for change in changes {
            let (position, rotation) = change;
            if let Some(pos) = position {
                self.set_position(pos);
            }
            if let Some(rot) = rotation {
                self.set_rotation(rot);
            }
        }
    }

    /// Any actor-specific update code (overridable)
    fn update_actor(&mut self, delta_time: f32);

    // ProcessInput function called from Game (not overridable)
    fn process_input(&mut self, key_state: &KeyboardState) {
        if *self.get_state() != State::Active {
            return;
        }
        for component in self.get_cocmponents() {
            component.borrow_mut().process_input(&key_state);
        }
        self.actor_input(&key_state);
    }

    // Any actor-specific input code (overridable)
    fn actor_input(&mut self, _key_state: &KeyboardState) {}

    /// Getters/setters
    fn get_forward(&self) -> Vector2;

    fn get_position(&self) -> &Vector2;

    fn set_position(&mut self, position: Vector2);

    fn get_scale(&self) -> f32;

    fn set_scale(&mut self, scale: f32);

    fn get_rotation(&self) -> f32;

    fn set_rotation(&mut self, rotation: f32);

    fn get_state(&self) -> &State;

    fn set_state(&mut self, state: State);

    fn get_texture_manager(&self) -> &Rc<RefCell<TextureManager>>;

    fn get_entity_manager(&self) -> &Rc<RefCell<EntityManager>>;

    fn get_cocmponents(&self) -> &Vec<Rc<RefCell<dyn Component>>>;

    fn clear_components(&mut self);

    /// Add/remove components
    fn add_component(&mut self, component: Rc<RefCell<dyn Component>>);

    fn remove_component(&mut self, component: Rc<RefCell<dyn Component>>);
}

macro_rules! impl_getters_setters {
    () => {
        fn get_forward(&self) -> Vector2 {
            Vector2::new(self.rotation.cos(), -self.rotation.sin())
        }

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

        fn get_cocmponents(&self) -> &Vec<Rc<RefCell<dyn Component>>> {
            &self.components
        }

        fn clear_components(&mut self) {
            self.components.clear();
        }

        fn get_texture_manager(&self) -> &Rc<RefCell<TextureManager>> {
            cfg_if::cfg_if! {
                if #[cfg(not(test))] {
                    &self.texture_manager
                } else {
                    panic!();
                }
            }
        }

        fn get_entity_manager(&self) -> &Rc<RefCell<EntityManager>> {
            cfg_if::cfg_if! {
                if #[cfg(not(test))] {
                    &self.entity_manager
                } else {
                    panic!();
                }
            }
        }
    };
}

pub(crate) use impl_getters_setters;

macro_rules! impl_component_operation {
    () => {
        fn add_component(&mut self, component: Rc<RefCell<dyn Component>>) {
            debug_assert!(*component.borrow().get_state() == ComponentState::Active);
            self.components.push(component);
        }

        fn remove_component(&mut self, component: Rc<RefCell<dyn Component>>) {
            debug_assert!(*component.borrow().get_state() == ComponentState::Dead);
            self.components
                .retain(|c| c.borrow().get_id() != component.borrow().get_id());
        }
    };
}

pub(crate) use impl_component_operation;

pub fn remove_actor(actor: Rc<RefCell<dyn Actor>>) {
    actor.borrow_mut().set_state(State::Dead);
    for component in actor.borrow().get_cocmponents() {
        component.borrow_mut().set_state(ComponentState::Dead);
    }
    actor.borrow_mut().clear_components();
}

macro_rules! impl_drop {
    () => {
        fn drop(&mut self) {
            for component in &self.components {
                component.borrow_mut().set_state(ComponentState::Dead);
            }
            self.components.clear();
        }
    };
}

pub(crate) use impl_drop;
use sdl2::keyboard::KeyboardState;

pub struct DefaultActor {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
}

impl DefaultActor {
    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            texture_manager,
            entity_manager: entity_manager.clone(),
        };

        let result = Rc::new(RefCell::new(this));

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }
}

impl Actor for DefaultActor {
    fn update_actor(&mut self, _delta_time: f32) {}

    impl_getters_setters! {}

    impl_component_operation! {}
}

impl Drop for DefaultActor {
    impl_drop! {}
}

#[cfg(test)]
pub mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        assert_near_eq,
        components::component::{tests::TestComponent, Component, State as ComponentState},
        math::{self, vector2::Vector2},
        system::{entity_manager::EntityManager, texture_manager::TextureManager},
    };

    use super::{Actor, State};

    pub struct TestActor {
        state: State,
        position: Vector2,
        scale: f32,
        rotation: f32,
        components: Vec<Rc<RefCell<dyn Component>>>,
    }

    impl TestActor {
        pub fn new() -> Self {
            Self {
                state: State::Active,
                position: Vector2::ZERO,
                scale: 1.0,
                rotation: 0.0,
                components: vec![],
            }
        }
    }

    impl Actor for TestActor {
        fn update_actor(&mut self, _delta_time: f32) {}

        impl_getters_setters! {}

        impl_component_operation! {}
    }

    impl Drop for TestActor {
        impl_drop! {}
    }

    #[test]
    fn test_remove_component() {
        let test_actor = TestActor::new();
        let mut owner: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor));
        let test_component0 = TestComponent::new(&mut owner, 100);
        let test_component1 = TestComponent::new(&mut owner, 100);

        test_component0.borrow_mut().set_state(ComponentState::Dead);
        owner.borrow_mut().remove_component(test_component0);

        let binding = owner.borrow();
        let actual = binding.get_cocmponents()[0].borrow();

        assert_eq!(1, binding.get_cocmponents().len());
        assert_eq!(test_component1.borrow().get_id(), actual.get_id());
    }

    #[test]
    fn test_get_forward() {
        let expected = Vector2::new(1.0 / 2.0, -3.0_f32.sqrt() / 2.0);

        let radian = math::basic::to_radians(60.0);
        let mut test_actor = TestActor::new();
        test_actor.set_rotation(radian);
        let actual = test_actor.get_forward();

        assert_near_eq!(expected.x, actual.x, 0.001);
        assert_near_eq!(expected.y, actual.y, 0.001);
    }
}
