use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::actors::actor::Actor;

static ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Active,
    Dead,
}

pub trait Component {
    fn update(&mut self, delta_time: f32);

    fn get_id(&self) -> u32;

    fn get_update_order(&self) -> i32;

    fn get_owner(&self) -> &Rc<RefCell<dyn Actor>>;

    fn get_state(&self) -> &State;

    fn set_state(&mut self, state: State);
}

pub fn generate_id() -> u32 {
    let id = ID.load(Ordering::SeqCst);
    ID.fetch_add(1, Ordering::SeqCst);
    id
}

macro_rules! impl_getters_setters {
    () => {
        fn get_id(&self) -> u32 {
            self.id
        }

        fn get_update_order(&self) -> i32 {
            self.update_order
        }

        fn get_owner(&self) -> &Rc<RefCell<dyn Actor>> {
            &self.owner
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn set_state(&mut self, state: State) {
            self.state = state;
        }
    };
}

pub(crate) use impl_getters_setters;

pub fn remove_component(this: Rc<RefCell<dyn Component>>) {
    debug_assert!(*this.borrow().get_state() == State::Active, "not active");
    this.borrow_mut().set_state(State::Dead);
    this.borrow()
        .get_owner()
        .borrow_mut()
        .remove_component(this.clone());
}

#[cfg(test)]
pub mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        actors::actor::{test::TestActor, Actor},
        components::component::remove_component,
    };

    use super::{generate_id, Component, State};

    pub struct TestComponent {
        id: u32,
        owner: Rc<RefCell<dyn Actor>>,
        update_order: i32,
        state: State,
    }

    impl TestComponent {
        pub fn new(
            owner: &mut Rc<RefCell<dyn Actor>>,
            update_order: i32,
        ) -> Rc<RefCell<dyn Component>> {
            let this = Self {
                id: generate_id(),
                owner: owner.clone(),
                update_order,
                state: State::Active,
            };
            let result = Rc::new(RefCell::new(this));
            owner.borrow_mut().add_component(result.clone());
            result
        }
    }

    impl Component for TestComponent {
        fn update(&mut self, delta_time: f32) {}

        impl_getters_setters! {}
    }

    #[test]
    fn test_new() {
        let test_actor = TestActor::new();
        let mut owner: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor));
        let _test_component = TestComponent::new(&mut owner, 100);

        assert_eq!(1, owner.borrow().get_cocmponents().len());
    }

    #[test]
    fn test_remove() {
        let test_actor = TestActor::new();
        let mut owner: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor));
        let test_component = TestComponent::new(&mut owner, 100);
        remove_component(test_component);

        assert_eq!(0, owner.borrow().get_cocmponents().len());
    }
}
