use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::{actor::Actor, tile::Tile},
    math::{self, vector2::Vector2},
};

use super::{
    component::{self, generate_id, Component, State},
    move_component::{self, MoveComponent},
};

pub struct NavComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    owner_position: Vector2,
    update_order: i32,
    state: State,
    angular_speed: f32,
    forward_speed: f32,
    next_node: Option<Rc<RefCell<Tile>>>,
}

impl NavComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>, update_order: i32) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            owner_position: owner.borrow().get_position().clone(),
            update_order: update_order,
            state: State::Active,
            angular_speed: 0.0,
            forward_speed: 0.0,
            next_node: None,
        };

        let result = Rc::new(RefCell::new(this));

        let mut borrowed_onwer = owner.borrow_mut();
        borrowed_onwer.add_component(result.clone());

        result
    }

    pub fn turn_to(&self, target_position: &Vector2) -> f32 {
        let actor_to_target = target_position.clone() - self.owner_position.clone();
        let angle = (-actor_to_target.y).atan2(actor_to_target.x);
        angle
    }

    pub fn start_path(start: Rc<RefCell<Tile>>) {
        unimplemented!()
    }
}

impl MoveComponent for NavComponent {
    move_component::impl_getters_setters! {}
}

impl Component for NavComponent {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector2, f32, Vector2),
    ) -> (Option<Vector2>, Option<f32>) {
        self.owner_position = owner_info.0.clone();

        let mut result = (None, None);

        let next_node = self.next_node.clone();
        if let Some(next_node) = next_node {
            let diff = owner_info.0.clone() - next_node.borrow().get_position().clone();
            if math::basic::near_zero(diff.length(), 2.0) {
                self.next_node = next_node.borrow().get_parent().clone();
                let angle = self.turn_to(self.next_node.clone().unwrap().borrow().get_position());
                result.1 = Some(angle);
            }
        }

        move_component::update_move_component(self, delta_time, owner_info, result)
    }

    component::impl_getters_setters! {}
}
