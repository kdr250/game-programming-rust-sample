use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        circle_component::CircleComponent,
        component::{Component, State as ComponentState},
        move_component::{DefaultMoveComponent, MoveComponent},
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::{matrix4::Matrix4, quaternion::Quaternion, vector2::Vector2, vector3::Vector3},
    system::{asset_manager::AssetManager, entity_manager::EntityManager},
};

use super::actor::{self, generate_id, Actor, State};

pub struct Laser {
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
    circle: Option<Rc<RefCell<CircleComponent>>>,
    death_timer: f32,
}

impl Laser {
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
            asset_manager: asset_manager.clone(),
            entity_manager: entity_manager.clone(),
            circle: None,
            death_timer: 1.0,
        };

        let result = Rc::new(RefCell::new(this));

        // Create a sprite component
        let sprite_component: Rc<RefCell<dyn SpriteComponent>> =
            DefaultSpriteComponent::new(result.clone(), 100);
        let texture = asset_manager.borrow_mut().get_texture("Laser.png");
        sprite_component.borrow_mut().set_texture(texture);

        // Create a move component, and set a forward speed
        let move_component: Rc<RefCell<dyn MoveComponent>> =
            DefaultMoveComponent::new(result.clone());
        move_component.borrow_mut().set_forward_speed(800.0);

        // Create a circle component (for collision)
        let circle = CircleComponent::new(result.clone());
        circle.borrow_mut().set_radius(11.0);
        result.borrow_mut().circle = Some(circle);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }
}

impl Actor for Laser {
    fn update_actor(&mut self, delta_time: f32) {
        self.death_timer -= delta_time;
        if self.death_timer <= 0.0 {
            self.set_state(State::Dead);
            return;
        }

        let mut is_dead = false;
        let binding = self.circle.clone().unwrap();
        let circle = binding.borrow();

        for asteroid in self.entity_manager.borrow().get_asteroids() {
            let mut borrowed_asteroid = asteroid.borrow_mut();
            if circle.intersect(borrowed_asteroid.get_circle()) {
                is_dead = true;
                borrowed_asteroid.set_state(State::Dead);
                break;
            }
        }

        if is_dead {
            self.set_state(State::Dead);
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Laser {
    actor::impl_drop! {}
}
