use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    collision::{aabb::AABB, line_segment::LineSegment},
    components::{
        box_component::BoxComponent,
        component::{Component, State},
    },
    math::vector3::Vector3,
};

pub struct CollisionInfo {
    // Point of collision
    pub point: Vector3,
    // Normal at collision
    pub normal: Vector3,
    // Component collided with
    pub box_component: Rc<RefCell<BoxComponent>>,
    // Owning actor of component
    pub actor: Rc<RefCell<dyn Actor>>,
    pub actor_id: u32,
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
        let mut closest_t = f32::INFINITY;
        let mut result = None;

        for b in &self.boxes {
            if let Some((t, normal)) = LineSegment::intersect_aabb(line, b.borrow().get_world_box())
            {
                if t < closest_t {
                    closest_t = t;
                    let collision_info = CollisionInfo {
                        point: line.point_on_segment(t),
                        normal,
                        box_component: b.clone(),
                        actor: b.borrow().get_owner().clone(),
                        actor_id: b.borrow().get_owner_id(),
                    };
                    result = Some(collision_info);
                }
            }
        }

        result
    }

    #[deprecated = "Naive implementation O(n^2). Not effecient..."]
    pub fn test_pairwise(&self, f: fn(Rc<RefCell<dyn Actor>>, Rc<RefCell<dyn Actor>>)) {
        for i in 0..self.boxes.len() {
            // Don't need to test vs itself and any previous i values
            for j in (i + 1)..self.boxes.len() {
                let a = &self.boxes[i];
                let b = &self.boxes[j];
                if AABB::intersect(a.borrow().get_world_box(), b.borrow().get_world_box()) {
                    // Call supplied function to handle intersection
                    f(
                        a.borrow().get_owner().clone(),
                        b.borrow().get_owner().clone(),
                    );
                }
            }
        }
    }

    /// Test collisions using sweep and prune
    pub fn test_sweep_and_prune(&mut self, f: fn(Rc<RefCell<dyn Actor>>, Rc<RefCell<dyn Actor>>)) {
        // Sort by min.x
        self.boxes.sort_by(|a, b| {
            a.borrow()
                .get_world_box()
                .min
                .x
                .partial_cmp(&b.borrow().get_world_box().min.x)
                .unwrap()
        });

        for i in 0..self.boxes.len() {
            // Get max.x for current box
            let a = &self.boxes[i];
            let max = a.borrow().get_world_box().max.x;
            for j in (i + 1)..self.boxes.len() {
                // If AABB[j] min is past the max bounds of AABB[i],
                // then there aren't any other possible intersections against AABB[i]
                let b = &self.boxes[j];
                if b.borrow().get_world_box().min.x > max {
                    break;
                }
                if AABB::intersect(a.borrow().get_world_box(), b.borrow().get_world_box()) {
                    f(
                        a.borrow().get_owner().clone(),
                        b.borrow().get_owner().clone(),
                    );
                }
            }
        }
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
