use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        component::{Component, State as ComponentState},
        move_component::DefaultMoveComponent,
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::vector2::Vector2,
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::{
    actor::{self, generate_id, Actor, State},
    bullet::Bullet,
};

pub struct Tower {
    id: u32,
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    movement: Option<Rc<RefCell<DefaultMoveComponent>>>,
    next_attack: f32,
}

impl Tower {
    const ATTACK_TIME: f32 = 2.5;
    const ATTACK_RANGE: f32 = 100.0;

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
            movement: None,
            next_attack: Tower::ATTACK_TIME,
        };

        let result = Rc::new(RefCell::new(this));

        let sprite_component = DefaultSpriteComponent::new(result.clone(), 100);
        let texture = texture_manager.borrow_mut().get_texture("Assets/Tower.png");
        sprite_component.borrow_mut().set_texture(texture);

        let movement = DefaultMoveComponent::new(result.clone());
        result.borrow_mut().movement = Some(movement);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }
}

impl Actor for Tower {
    fn update_actor(&mut self, delta_time: f32) {
        self.next_attack -= delta_time;
        if self.next_attack <= 0.0 {
            let enemy = self
                .entity_manager
                .borrow()
                .get_nearest_enemy(&self.position);

            if let Some(enemy) = enemy {
                let tower_to_enemy = enemy.borrow().get_position().clone() - self.position.clone();
                let distance = tower_to_enemy.length();
                if distance < Tower::ATTACK_RANGE {
                    self.set_rotation((-tower_to_enemy.y).atan2(tower_to_enemy.x));
                    let bullet =
                        Bullet::new(self.texture_manager.clone(), self.entity_manager.clone());
                    bullet.borrow_mut().set_position(self.position.clone());
                    bullet.borrow_mut().set_rotation(self.rotation);
                }
            }

            self.next_attack += Tower::ATTACK_TIME;
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Tower {
    actor::impl_drop! {}
}
