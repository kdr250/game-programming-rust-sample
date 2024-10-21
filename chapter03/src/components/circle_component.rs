use std::{cell::RefCell, rc::Rc};

use crate::{actors::actor::Actor, math::vector2::Vector2};

use super::component::{self, generate_id, Component, State};

pub struct CircleComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    center: Vector2,
    radius: f32,
}

impl CircleComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 100,
            state: State::Active,
            center: owner.borrow().get_position().clone(),
            radius: 0.0,
        };
        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        result
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_center(&self) -> &Vector2 {
        &self.center
    }

    pub fn intersect(&self, other: Rc<RefCell<CircleComponent>>) -> bool {
        let borrowed_other = other.borrow();
        let diff = self.center.clone() - borrowed_other.center.clone();
        let distance_sq = diff.length_sqrt();

        let radius_diff = self.radius + borrowed_other.radius;
        let radius_sq = radius_diff * radius_diff;

        distance_sq <= radius_sq
    }
}

impl Component for CircleComponent {
    fn update(
        &mut self,
        _delta_time: f32,
        _owner_info: &(Vector2, f32, Vector2),
    ) -> (Option<Vector2>, Option<f32>) {
        (None, None)
    }

    component::impl_getters_setters! {}
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        actors::actor::{test::TestActor, Actor},
        math::vector2::Vector2,
    };

    use super::CircleComponent;

    #[test]
    fn test_intersect_true() {
        let mut test_actor1 = TestActor::new();
        test_actor1.set_position(Vector2::new(0.0, 0.0));
        let owner1: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor1));
        let circle1 = CircleComponent::new(owner1);
        circle1.borrow_mut().set_radius(5.0);

        let mut test_actor2 = TestActor::new();
        test_actor2.set_position(Vector2::new(7.0, 7.0));
        let owner2: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor2));
        let circle2 = CircleComponent::new(owner2);
        circle2.borrow_mut().set_radius(5.0);

        let result = circle1.borrow().intersect(circle2);

        assert!(result);
    }

    #[test]
    fn test_intersect_false() {
        let mut test_actor1 = TestActor::new();
        test_actor1.set_position(Vector2::new(0.0, 0.0));
        let owner1: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor1));
        let circle1 = CircleComponent::new(owner1);
        circle1.borrow_mut().set_radius(5.0);

        let mut test_actor2 = TestActor::new();
        test_actor2.set_position(Vector2::new(8.0, 8.0));
        let owner2: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor2));
        let circle2 = CircleComponent::new(owner2);
        circle2.borrow_mut().set_radius(5.0);

        let result = !circle1.borrow().intersect(circle2);

        assert!(result);
    }
}
