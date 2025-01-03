use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    collision::line_segment::LineSegment,
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::phys_world::PhysWorld,
};

use super::{
    component::{self, generate_id, Component, State},
    move_component::{self, MoveComponent},
};

pub struct BallMove {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    angular_speed: f32,
    forward_speed: f32,
    strafe_speed: f32,
    phys_world: Rc<RefCell<PhysWorld>>,
    player_id: u32,
}

impl BallMove {
    const SEGMENT_LENGTH: f32 = 30.0;

    pub fn new(
        owner: Rc<RefCell<dyn Actor>>,
        phys_world: Rc<RefCell<PhysWorld>>,
        player_id: u32,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 10,
            state: State::Active,
            angular_speed: 0.0,
            forward_speed: 0.0,
            strafe_speed: 0.0,
            phys_world,
            player_id,
        };

        let result = Rc::new(RefCell::new(this));

        let mut borrowed_onwer = owner.borrow_mut();
        borrowed_onwer.add_component(result.clone());

        result
    }
}

impl MoveComponent for BallMove {
    move_component::impl_getters_setters! {}
}

impl Component for BallMove {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (
        Option<Vector3>,
        Option<Quaternion>,
        Option<Vector3>,
        Vec<Rc<RefCell<dyn Actor>>>,
    ) {
        // Construct segment in direction of travel
        let start = owner_info.0.clone();
        let mut direction = owner_info.2.clone();
        let end = start.clone() + direction.clone() * BallMove::SEGMENT_LENGTH;

        // Create line segment
        let line = LineSegment::new(start, end);

        // Test segment vs world
        let mut hit_actors = vec![];
        if let Some(collision_info) = self.phys_world.borrow().segment_cast(&line) {
            if collision_info.actor_id != self.player_id {
                direction = Vector3::reflect(&direction, &collision_info.normal);
                hit_actors.push(collision_info.actor);
            }
        }

        let mut result = move_component::update_move_component(self, delta_time, owner_info);
        result.2 = Some(direction);
        result.3 = hit_actors;

        result
    }

    component::impl_getters_setters! {}
}
