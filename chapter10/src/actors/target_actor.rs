use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        box_component::BoxComponent,
        component::{Component, State as ComponentState},
        mesh_component::MeshComponent,
    },
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{asset_manager::AssetManager, entity_manager::EntityManager, phys_world::PhysWorld},
};

use super::actor::{self, generate_id, Actor, State};

pub struct TargetActor {
    id: u32,
    state: State,
    world_transform: Matrix4,
    recompute_world_transform: bool,
    position: Vector3,
    scale: f32,
    rotation: Quaternion,
    components: Vec<Rc<RefCell<dyn Component>>>,
    asset_manager: Rc<RefCell<AssetManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
}

impl TargetActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
        phys_world: Rc<RefCell<PhysWorld>>,
    ) -> Rc<RefCell<Self>> {
        let mut this = Self {
            id: generate_id(),
            state: State::Active,
            world_transform: Matrix4::new(),
            recompute_world_transform: true,
            position: Vector3::ZERO,
            scale: 1.0,
            rotation: Quaternion::new(),
            components: vec![],
            asset_manager: asset_manager.clone(),
            entity_manager: entity_manager.clone(),
        };

        this.set_rotation(Quaternion::from_axis_angle(
            &Vector3::UNIT_Z,
            f32::consts::PI,
        ));

        let result = Rc::new(RefCell::new(this));

        let mesh_component = MeshComponent::new(result.clone());
        let mesh = asset_manager.borrow_mut().get_mesh("Target.gpmesh");
        mesh_component.borrow_mut().set_mesh(mesh.clone());

        let box_component = BoxComponent::new(result.clone(), phys_world);
        box_component
            .borrow_mut()
            .set_object_box(mesh.get_box().clone());

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }
}

impl Actor for TargetActor {
    fn update_actor(&mut self, _delta_time: f32) {}

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for TargetActor {
    actor::impl_drop! {}
}
