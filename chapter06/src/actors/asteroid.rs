use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        circle_component::CircleComponent,
        component::{Component, State as ComponentState},
        move_component::{DefaultMoveComponent, MoveComponent},
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::actor::{self, generate_id, Actor, State};

pub struct Asteroid {
    id: u32,
    state: State,
    world_transform: Matrix4,
    recompute_world_transform: bool,
    position: Vector3,
    scale: f32,
    rotation: Quaternion,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    circle: Option<Rc<RefCell<CircleComponent>>>,
}

impl Asteroid {
    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let mut this = Self {
            id: generate_id(),
            state: State::Active,
            world_transform: Matrix4::new(),
            recompute_world_transform: true,
            position: Vector3::ZERO,
            scale: 1.0,
            rotation: Quaternion::new(),
            components: vec![],
            texture_manager: texture_manager.clone(),
            entity_manager: entity_manager.clone(),
            circle: None,
        };

        // Initialize to random position/orientation
        {
            let mut borrowed_entity_manager = entity_manager.borrow_mut();
            let random = borrowed_entity_manager.get_random();
            let random_position = random.get_vector3(
                Vector3::new(-512.0, -384.0, 0.0),
                Vector3::new(512.0, 384.0, 0.0),
            );
            let random_rotation = random.get_float_range(0.0, f32::consts::TAU);
            let random_rotation = Quaternion::from_axis_angle(&Vector3::UNIT_Z, random_rotation);
            this.set_position(random_position);
            this.set_rotation(random_rotation);
        }

        let result = Rc::new(RefCell::new(this));
        entity_manager.borrow_mut().add_actor(result.clone());

        // Create a sprite component
        let sprite_component = DefaultSpriteComponent::new(result.clone(), 100);
        let texture = texture_manager.borrow_mut().get_texture("Asteroid.png");
        sprite_component.borrow_mut().set_texture(texture);

        // Create a move component, and set a forward speed
        let move_component: Rc<RefCell<dyn MoveComponent>> =
            DefaultMoveComponent::new(result.clone());
        move_component.borrow_mut().set_forward_speed(150.0);

        // Create a circle component (for collision)
        let circle = CircleComponent::new(result.clone());
        circle.borrow_mut().set_radius(40.0);
        result.borrow_mut().circle = Some(circle);

        result
    }

    pub fn get_circle(&self) -> Rc<RefCell<CircleComponent>> {
        self.circle.clone().unwrap()
    }
}

impl Actor for Asteroid {
    fn update_actor(&mut self, _delta_time: f32) {}

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Asteroid {
    actor::impl_drop! {}
}
