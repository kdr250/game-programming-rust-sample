use std::{cell::RefCell, rc::Rc};

use crate::actor::Actor;

pub trait Component {
    fn update(&mut self, delta_time: f32);

    fn get_update_order(&self) -> i32;

    fn get_owner(&self) -> &Rc<RefCell<dyn Actor>>;
}

macro_rules! impl_new {
    () => {
        fn new(owner: &mut Rc<RefCell<dyn Actor>>, update_order: i32) -> Self {
            let this = Self {
                owner: owner.clone(),
                update_order,
            };
            owner.borrow_mut().add_component(&this);
            this
        }
    };
}

/// impl Drop for XXXComponent
macro_rules! impl_drop {
    () => {
        fn drop(&mut self) {
            self.owner.borrow_mut().remove_component(self);
        }
    };
}
