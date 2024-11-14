use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    collision::line_segment::LineSegment,
    components::{
        box_component::BoxComponent,
        component::{Component, State},
    },
    math::vector3::Vector3,
};

pub struct CollisionInfo {
    // Point of collision
    point: Vector3,
    // Normal at collision
    normal: Vector3,
    // Component collided with
    box_component: Rc<RefCell<BoxComponent>>,
    // Owning actor of component
    actor: Rc<RefCell<dyn Actor>>,
}

pub struct PhysWorld {
    boxes: Vec<Rc<RefCell<BoxComponent>>>,
}

impl PhysWorld {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self { boxes: vec![] };
        Rc::new(RefCell::new(this))
    }

    /// Test a line segment against boxes
    /// Returns Some(CollisionInfo) if it collides against a box
    pub fn segment_cast(&self, line: &LineSegment) -> Option<CollisionInfo> {
        let closest_t = f32::INFINITY;
        let mut result = None;

        for b in &self.boxes {
            if let Some((t, normal)) = LineSegment::intersect_aabb(line, b.borrow().get_world_box())
            {
                if t < closest_t {
                    let collision_info = CollisionInfo {
                        point: line.point_on_segment(t),
                        normal,
                        box_component: b.clone(),
                        actor: b.borrow().get_owner().clone(),
                    };
                    result = Some(collision_info);
                }
            }
        }

        result
    }

    /// Add box components from world
    pub fn add_box(&mut self, box_component: Rc<RefCell<BoxComponent>>) {
        self.boxes.push(box_component);
    }

    /// Remove box components from world
    pub fn remove_box(&mut self, box_component: &Rc<RefCell<BoxComponent>>) {
        let id = box_component.borrow().get_id();
        self.boxes.retain(|b| b.borrow().get_id() != id);
    }

    pub fn flush_boxes(&mut self) {
        self.boxes
            .retain(|b| *b.borrow().get_state() == State::Active);
    }
}
