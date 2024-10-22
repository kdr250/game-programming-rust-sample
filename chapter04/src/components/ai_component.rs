use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{actors::actor::Actor, math::vector2::Vector2};

use super::{
    ai_state::AIState,
    component::{self, generate_id, Component, State},
};

pub struct AIComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    state_map: HashMap<String, Rc<RefCell<dyn AIState>>>,
    current_state: Option<Rc<RefCell<dyn AIState>>>,
}

impl AIComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>) -> Rc<RefCell<AIComponent>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 100,
            state: State::Active,
            state_map: HashMap::new(),
            current_state: None,
        };

        let result = Rc::new(RefCell::new(this));

        owner.borrow_mut().add_component(result.clone());

        result
    }

    fn change_state(&mut self, name: &str) {
        // First exit the current state
        if let Some(ai_state) = &self.current_state {
            ai_state.borrow_mut().on_exit();
        }

        // Try to find the new state from the map
        if let Some(next_state) = self.state_map.get(&name.to_string()) {
            next_state.borrow_mut().on_enter();
            self.current_state = Some(next_state.clone());
        } else {
            self.current_state = None;
        }
    }

    fn register_state(&mut self, state: Rc<RefCell<dyn AIState>>) {
        let name = state.borrow().get_name().clone();
        self.state_map.insert(name, state);
    }
}

impl Component for AIComponent {
    fn update(
        &mut self,
        delta_time: f32,
        _owner_info: &(Vector2, f32, Vector2),
    ) -> (Option<Vector2>, Option<f32>) {
        let mut update_result = None;
        if let Some(ai_state) = &self.current_state {
            update_result = ai_state.borrow_mut().update(delta_time);
        }
        if let Some(name) = update_result {
            self.change_state(name.as_str());
        }
        (None, None)
    }

    component::impl_getters_setters! {}
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        actors::actor::test::TestActor,
        components::{
            ai_state::{AIAttack, AIDeath, AIPatrol},
            component::Component,
        },
        math::vector2::Vector2,
    };

    use super::AIComponent;

    #[test]
    fn test_change_state() {
        let test_actor = TestActor::new();
        let owner = Rc::new(RefCell::new(test_actor));
        let ai_component = AIComponent::new(owner);

        ai_component.borrow_mut().register_state(AIPatrol::new());
        ai_component.borrow_mut().register_state(AIDeath::new());
        ai_component.borrow_mut().register_state(AIAttack::new());

        ai_component.borrow_mut().change_state("Patrol");

        let binding = ai_component.borrow().current_state.clone().unwrap();
        let binding = binding.borrow_mut();
        let actual = binding.get_name().as_str();

        assert_eq!("Patrol", actual)
    }

    #[test]
    fn test_update() {
        let test_actor = TestActor::new();
        let owner = Rc::new(RefCell::new(test_actor));
        let ai_component = AIComponent::new(owner);

        ai_component.borrow_mut().register_state(AIPatrol::new());
        ai_component.borrow_mut().register_state(AIDeath::new());
        ai_component.borrow_mut().register_state(AIAttack::new());

        ai_component.borrow_mut().change_state("Patrol");
        ai_component
            .borrow_mut()
            .update(0.0, &(Vector2::ZERO, 0.0, Vector2::ZERO));

        let binding = ai_component.borrow().current_state.clone().unwrap();
        let binding = binding.borrow_mut();
        let actual = binding.get_name().as_str();

        assert_eq!("Death", actual)
    }
}
