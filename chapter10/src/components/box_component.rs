use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    collision::aabb::AABB,
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::phys_world::PhysWorld,
};

use super::component::{self, generate_id, Component, State};

pub struct BoxComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    owner_id: u32,
    update_order: i32,
    state: State,
    object_box: AABB,
    world_box: AABB,
    should_rotate: bool,
}

impl BoxComponent {
    pub fn new(
        owner: Rc<RefCell<dyn Actor>>,
        phys_world: Rc<RefCell<PhysWorld>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            owner_id: owner.borrow().get_id(),
            update_order: 100,
            state: State::Active,
            object_box: AABB::new(Vector3::ZERO, Vector3::ZERO),
            world_box: AABB::new(Vector3::ZERO, Vector3::ZERO),
            should_rotate: true,
        };

        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        phys_world.borrow_mut().add_box(result.clone());

        result
    }

    pub fn set_object_box(&mut self, model: AABB) {
        self.object_box = model;
    }

    pub fn get_world_box(&self) -> &AABB {
        &self.world_box
    }

    pub fn set_should_rotate(&mut self, value: bool) {
        self.should_rotate = value;
    }

    pub fn get_owner_id(&self) -> u32 {
        self.owner_id
    }
}

impl Component for BoxComponent {
    fn update(
        &mut self,
        _delta_time: f32,
        _owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (
        Option<Vector3>,
        Option<Quaternion>,
        Option<Vector3>,
        Vec<Rc<RefCell<dyn Actor>>>,
    ) {
        (None, None, None, vec![])
    }

    fn on_update_world_transform(&mut self, owner_info: &(Vector3, f32, Quaternion)) {
        // Reset to object space box
        self.world_box = self.object_box.clone();

        // Scale
        self.world_box.min *= owner_info.1;
        self.world_box.max *= owner_info.1;

        // Rotate (if we want to)
        if self.should_rotate {
            self.world_box.rotate(&owner_info.2);
        }

        // Translate
        self.world_box.min += owner_info.0.clone();
        self.world_box.max += owner_info.0.clone();
    }

    component::impl_getters_setters! {}
}
