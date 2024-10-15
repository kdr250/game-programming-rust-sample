use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicU32};

use crate::actor::Actor;

static ID: AtomicU32 = AtomicU32::new(0);

pub trait Component {
    fn id(&self) -> u32;

    fn update(&mut self, delta_time: f32);

    fn get_update_order(&self) -> i32;

    fn get_owner(&self) -> &Rc<RefCell<dyn Actor>>;
}

macro_rules! impl_new {
    () => {
        fn new(
            owner: &mut Rc<RefCell<dyn Actor>>,
            update_order: i32,
        ) -> Rc<RefCell<dyn Component>> {
            use crate::component::ID;
            use std::sync::atomic::Ordering;
            let this = Self {
                id: ID.load(Ordering::SeqCst),
                owner: owner.clone(),
                update_order,
            };
            ID.fetch_add(1, Ordering::SeqCst);
            let result = Rc::new(RefCell::new(this));
            owner.borrow_mut().add_component(result.clone());
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::actor::{test::TestActor, Actor};

    use super::Component;

    struct TestComponent {
        id: u32,
        owner: Rc<RefCell<dyn Actor>>,
        update_order: i32,
    }

    impl TestComponent {
        impl_new! {}
    }

    impl Component for TestComponent {
        fn id(&self) -> u32 {
            self.id
        }

        fn update(&mut self, delta_time: f32) {}

        fn get_update_order(&self) -> i32 {
            self.update_order
        }

        fn get_owner(&self) -> &Rc<RefCell<dyn Actor>> {
            &self.owner
        }
    }

    #[test]
    fn test_new() {
        let test_actor = TestActor::new();
        let mut owner: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor));
        let _test_component = TestComponent::new(&mut owner, 100);

        assert_eq!(1, owner.borrow().get_cocmponents().len());
    }

    #[test]
    fn test_owner_remove_component() {
        let test_actor = TestActor::new();
        let mut owner: Rc<RefCell<dyn Actor>> = Rc::new(RefCell::new(test_actor));
        let test_component0 = TestComponent::new(&mut owner, 100);
        let test_component1 = TestComponent::new(&mut owner, 100);

        owner.borrow_mut().remove_component(test_component0);

        let binding = owner.borrow();
        let actual = binding.get_cocmponents()[0].borrow();

        assert_eq!(1, binding.get_cocmponents().len());
        assert_eq!(test_component1.borrow().id(), actual.id());
    }
}
