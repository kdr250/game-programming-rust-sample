use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        circle_component::CircleComponent,
        component::{Component, State as ComponentState},
        move_component::{DefaultMoveComponent, MoveComponent},
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::vector2::Vector2,
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::{
    actor::{self, generate_id, Actor, State},
    enemy::Enemy,
};

pub struct Bullet {
    id: u32,
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    circle: Option<Rc<RefCell<CircleComponent>>>,
    live_time: f32,
}

impl Bullet {
    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            texture_manager: texture_manager.clone(),
            entity_manager: entity_manager.clone(),
            circle: None,
            live_time: 1.0,
        };

        let result = Rc::new(RefCell::new(this));

        let sprite_component = DefaultSpriteComponent::new(result.clone(), 100);
        let texture = texture_manager
            .borrow_mut()
            .get_texture("Assets/Projectile.png");
        sprite_component.borrow_mut().set_texture(texture);

        let move_component = DefaultMoveComponent::new(result.clone());
        move_component.borrow_mut().set_forward_speed(400.0);

        let circle = CircleComponent::new(result.clone());
        circle.borrow_mut().set_radius(5.0);
        result.borrow_mut().circle = Some(circle);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }
}

impl Actor for Bullet {
    fn update_actor(&mut self, delta_time: f32) {
        let mut result = None;
        let circle = self.circle.clone().unwrap();
        for enemy in self.entity_manager.borrow().get_enemies() {
            if circle.borrow().intersect(enemy.borrow().get_circle()) {
                result = Some(enemy.clone());
                break;
            }
        }

        if let Some(enemy) = result {
            enemy.borrow_mut().set_state(State::Dead);
            self.set_state(State::Dead);
        }

        self.live_time -= delta_time;
        if self.live_time <= 0.0 {
            self.set_state(State::Dead);
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Bullet {
    actor::impl_drop! {}
}
