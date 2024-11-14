use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        audio_component::AudioComponent,
        ball_move::BallMove,
        component::{Component, State as ComponentState},
        mesh_component::MeshComponent,
        move_component::MoveComponent,
    },
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        phys_world::PhysWorld,
    },
};

use super::actor::{self, generate_id, Actor, State};

pub struct BallActor {
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
    audio_component: Option<Rc<RefCell<AudioComponent>>>,
    ball_move: Option<Rc<RefCell<BallMove>>>,
    life_span: f32,
}

impl BallActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
        phys_world: Rc<RefCell<PhysWorld>>,
        player: Rc<RefCell<dyn Actor>>,
        audio_system: Rc<RefCell<AudioSystem>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
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
            audio_component: None,
            ball_move: None,
            life_span: 2.0,
        };

        let result = Rc::new(RefCell::new(this));

        let mesh_component = MeshComponent::new(result.clone());
        let mesh = asset_manager.borrow_mut().get_mesh("Sphere.gpmesh");
        mesh_component.borrow_mut().set_mesh(mesh);

        let ball_move = BallMove::new(result.clone(), phys_world, player);
        ball_move.borrow_mut().set_forward_speed(1500.0);
        result.borrow_mut().ball_move = Some(ball_move);

        let audio_component = AudioComponent::new(result.clone(), audio_system);
        result.borrow_mut().audio_component = Some(audio_component);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn hit_target(&self) {
        let audio_component = self.audio_component.as_ref().unwrap();
        audio_component.borrow_mut().play_event("event:/Ding");
    }
}

impl Actor for BallActor {
    fn update_actor(&mut self, delta_time: f32) {
        self.life_span -= delta_time;
        if self.life_span < 0.0 {
            self.set_state(State::Dead);
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for BallActor {
    actor::impl_drop! {}
}
