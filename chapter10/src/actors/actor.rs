use core::f32;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::{
    components::component::{Component, State as ComponentState},
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{asset_manager::AssetManager, entity_manager::EntityManager},
};

static ID: AtomicU32 = AtomicU32::new(0);

pub fn generate_id() -> u32 {
    let id = ID.load(Ordering::SeqCst);
    ID.fetch_add(1, Ordering::SeqCst);
    id
}

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
            self.compute_world_transform();
            self.update_component(delta_time);
            self.update_actor(delta_time);
            self.compute_world_transform();
        }
    }

    /// Updates all the components attached to the actor (not overridable)
    fn update_component(&mut self, delta_time: f32) {
        let mut changes = vec![];
        let actor_info = (
            self.get_position().clone(),
            self.get_rotation().clone(),
            self.get_forward(),
            self.get_world_transform().clone(),
            self.get_right(),
        );

        for component in self.get_cocmponents() {
            let change = component.borrow_mut().update(delta_time, &actor_info);
            changes.push(change);
        }

        for change in changes {
            let (position, rotation, new_forward, hit_actors) = change;
            if let Some(forward) = new_forward {
                self.rotate_to_new_forward(forward);
            }
            if let Some(pos) = position {
                self.set_position(pos);
            }
            if let Some(rot) = rotation {
                self.set_rotation(rot);
            }
            for hit_actor in hit_actors {
                hit_actor.borrow().hit_target();
            }
        }
    }

    /// Any actor-specific update code (overridable)
    fn update_actor(&mut self, delta_time: f32);

    // ProcessInput function called from Game (not overridable)
    fn process_input(&mut self, key_state: &KeyboardState, mouse_state: &RelativeMouseState) {
        if *self.get_state() != State::Active {
            return;
        }
        for component in self.get_cocmponents() {
            component.borrow_mut().process_input(&key_state);
        }
        self.actor_input(key_state, mouse_state);
    }

    // Any actor-specific input code (overridable)
    fn actor_input(&mut self, _key_state: &KeyboardState, _mouse_state: &RelativeMouseState) {}

    fn compute_world_transform(&mut self) {
        if !self.get_recompute_world_transform() {
            return;
        }

        self.set_recompute_world_transform(false);

        // Scale, then rotate, then translate
        let mut world_transform = Matrix4::create_scale(self.get_scale());
        world_transform *= Matrix4::create_from_quaternion(self.get_rotation());
        world_transform *= Matrix4::create_translation(self.get_position());
        self.set_world_transform(world_transform);

        // Inform components world transform updated
        for component in self.get_cocmponents() {
            let owner_info = (
                self.get_position().clone(),
                self.get_scale(),
                self.get_rotation().clone(),
            );
            component
                .borrow_mut()
                .on_update_world_transform(&owner_info);
        }
    }

    fn rotate_to_new_forward(&mut self, forward: Vector3) {
        // Figure out difference between original (unit x) and new
        let dot = Vector3::dot(&Vector3::UNIT_X, &forward);
        let angle = dot.acos();

        if dot > 0.9999 {
            // Facing down X
            self.set_rotation(Quaternion::IDENTITY);
        } else if dot < -0.9999 {
            // Facing down -X
            self.set_rotation(Quaternion::from_axis_angle(
                &Vector3::UNIT_Z,
                f32::consts::PI,
            ));
        } else {
            // Rotate about axis from cross product
            let mut axis = Vector3::cross(&Vector3::UNIT_X, &forward);
            axis.normalize_mut();
            self.set_rotation(Quaternion::from_axis_angle(&axis, angle));
        }
    }

    /// Getters/setters
    fn get_id(&self) -> u32;

    fn get_forward(&self) -> Vector3;

    fn get_right(&self) -> Vector3;

    fn get_world_transform(&self) -> &Matrix4;

    fn set_world_transform(&mut self, world_transform: Matrix4);

    fn get_recompute_world_transform(&self) -> bool;

    fn set_recompute_world_transform(&mut self, recompute: bool);

    fn get_position(&self) -> &Vector3;

    fn set_position(&mut self, position: Vector3);

    fn get_scale(&self) -> f32;

    fn set_scale(&mut self, scale: f32);

    fn get_rotation(&self) -> &Quaternion;

    fn set_rotation(&mut self, rotation: Quaternion);

    fn get_state(&self) -> &State;

    fn set_state(&mut self, state: State);

    fn get_asset_manager(&self) -> &Rc<RefCell<AssetManager>>;

    fn get_entity_manager(&self) -> &Rc<RefCell<EntityManager>>;

    fn get_cocmponents(&self) -> &Vec<Rc<RefCell<dyn Component>>>;

    fn clear_components(&mut self);

    /// Add/remove components
    fn add_component(&mut self, component: Rc<RefCell<dyn Component>>);

    fn remove_component(&mut self, component: Rc<RefCell<dyn Component>>);

    fn hit_target(&self) {}
}

macro_rules! impl_getters_setters {
    () => {
        fn get_id(&self) -> u32 {
            self.id
        }

        fn get_forward(&self) -> Vector3 {
            Vector3::transform(&Vector3::UNIT_X, &self.rotation)
        }

        fn get_right(&self) -> Vector3 {
            Vector3::transform(&Vector3::UNIT_Y, &self.rotation)
        }

        fn get_world_transform(&self) -> &Matrix4 {
            &self.world_transform
        }

        fn set_world_transform(&mut self, world_transform: Matrix4) {
            self.world_transform = world_transform;
            self.recompute_world_transform = true;
        }

        fn get_recompute_world_transform(&self) -> bool {
            self.recompute_world_transform
        }

        fn set_recompute_world_transform(&mut self, recompute: bool) {
            self.recompute_world_transform = recompute;
        }

        fn get_position(&self) -> &Vector3 {
            &self.position
        }

        fn set_position(&mut self, position: Vector3) {
            self.position = position;
            self.recompute_world_transform = true;
        }

        fn get_scale(&self) -> f32 {
            self.scale
        }

        fn set_scale(&mut self, scale: f32) {
            self.scale = scale;
            self.recompute_world_transform = true;
        }

        fn get_rotation(&self) -> &Quaternion {
            &self.rotation
        }

        fn set_rotation(&mut self, rotation: Quaternion) {
            self.rotation = rotation;
            self.recompute_world_transform = true;
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

        fn get_asset_manager(&self) -> &Rc<RefCell<AssetManager>> {
            cfg_if::cfg_if! {
                if #[cfg(not(test))] {
                    &self.asset_manager
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
use sdl2::{keyboard::KeyboardState, mouse::RelativeMouseState};

pub struct DefaultActor {
    id: u32,
    state: State,
    world_transform: Matrix4,
    recompute_world_transform: bool,
    position: Vector3,
    scale: f32,
    rotation: Quaternion,
    components: Vec<Rc<RefCell<dyn Component>>>,
    asset_manager: Rc<RefCell<AssetManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
}

impl DefaultActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            state: State::Active,
            world_transform: Matrix4::new(),
            recompute_world_transform: true,
            position: Vector3::ZERO,
            scale: 1.0,
            rotation: Quaternion::new(),
            components: vec![],
            asset_manager,
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
        math::{
            self, matrix4::Matrix4, quaternion::Quaternion, vector2::Vector2, vector3::Vector3,
        },
        system::{asset_manager::AssetManager, entity_manager::EntityManager},
    };

    use super::{generate_id, Actor, State};

    pub struct TestActor {
        id: u32,
        state: State,
        world_transform: Matrix4,
        recompute_world_transform: bool,
        position: Vector3,
        scale: f32,
        rotation: Quaternion,
        components: Vec<Rc<RefCell<dyn Component>>>,
    }

    impl TestActor {
        pub fn new() -> Self {
            Self {
                id: generate_id(),
                state: State::Active,
                world_transform: Matrix4::new(),
                recompute_world_transform: true,
                position: Vector3::ZERO,
                scale: 1.0,
                rotation: Quaternion::new(),
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
        let expected = Vector2::new(1.0 / 2.0, 3.0_f32.sqrt() / 2.0);

        let radian = math::basic::to_radians(60.0);
        let rotation = Quaternion::from_axis_angle(&Vector3::UNIT_Z, radian);

        let mut test_actor = TestActor::new();
        test_actor.set_rotation(rotation);
        let actual = test_actor.get_forward();

        assert_near_eq!(expected.x, actual.x, 0.001);
        assert_near_eq!(expected.y, actual.y, 0.001);
    }
}
