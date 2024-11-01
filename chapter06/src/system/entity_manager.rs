use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::{
        actor::{self, Actor, DefaultActor, State as ActorState},
        camera_actor::CameraActor,
    },
    components::mesh_component::MeshComponent,
    math::{quaternion::Quaternion, random::Random, vector3::Vector3},
    system::{asset_manager::AssetManager, renderer::Renderer},
};

pub struct EntityManager {
    actors: Vec<Rc<RefCell<dyn Actor>>>,
    pending_actors: Vec<Rc<RefCell<dyn Actor>>>,
    updating_actors: bool,
    camera_actor: Option<Rc<RefCell<CameraActor>>>,
    random: Random,
}

impl EntityManager {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            actors: vec![],
            pending_actors: vec![],
            updating_actors: false,
            camera_actor: None,
            random: Random::new(),
        };

        Rc::new(RefCell::new(this))
    }

    pub fn add_actor(&mut self, actor: Rc<RefCell<dyn Actor>>) {
        if self.updating_actors {
            self.pending_actors.push(actor);
        } else {
            self.actors.push(actor);
        }
    }

    pub fn flush_actors(&mut self) {
        for pending in self.pending_actors.clone() {
            self.actors.push(pending);
        }
        self.pending_actors.clear();

        self.actors.retain(|actor| {
            if *actor.borrow().get_state() != ActorState::Dead {
                true
            } else {
                actor::remove_actor(actor.clone());
                false
            }
        });
    }

    pub fn load_data(
        this: Rc<RefCell<EntityManager>>,
        asset_manager: Rc<RefCell<AssetManager>>,
        renderer: Rc<RefCell<Renderer>>,
    ) {
        // Create actors
        let a = DefaultActor::new(asset_manager.clone(), this.clone());
        a.borrow_mut().set_position(Vector3::new(200.0, 75.0, 0.0));
        a.borrow_mut().set_scale(100.0);

        let mut q = Quaternion::from_axis_angle(&Vector3::UNIT_Y, -std::f32::consts::FRAC_PI_2);
        q = Quaternion::concatenate(
            &q,
            &Quaternion::from_axis_angle(&Vector3::UNIT_Z, f32::consts::PI + f32::consts::PI / 4.0),
        );
        a.borrow_mut().set_rotation(q);

        let mesh = MeshComponent::new(a.clone());
        mesh.borrow_mut()
            .set_mesh(asset_manager.borrow_mut().get_mesh("Cube.gpmesh"));

        let b = DefaultActor::new(asset_manager.clone(), this.clone());
        b.borrow_mut().set_position(Vector3::new(200.0, -75.0, 0.0));
        b.borrow_mut().set_scale(3.0);
        let mesh = MeshComponent::new(b.clone());
        mesh.borrow_mut()
            .set_mesh(asset_manager.borrow_mut().get_mesh("Sphere.gpmesh"));

        // Camera actor
        let camera_actor = CameraActor::new(asset_manager.clone(), this.clone(), renderer.clone());
        this.borrow_mut().camera_actor = Some(camera_actor);

        // Setup lights
        {
            let mut borrowed_renderer = renderer.borrow_mut();
            borrowed_renderer.set_ambient_light(Vector3::new(0.2, 0.2, 0.2));
            let directional_light = borrowed_renderer.get_directional_light_mut();
            directional_light.direction = Vector3::new(0.0, -0.707, -0.707);
            directional_light.diffuse_color = Vector3::new(0.78, 0.88, 1.0);
            directional_light.spec_color = Vector3::new(0.8, 0.8, 0.8);
        }

        // TODO: Setup floor
    }

    pub fn get_actors(&self) -> &Vec<Rc<RefCell<dyn Actor>>> {
        &self.actors
    }

    pub fn get_pending_actors(&self) -> &Vec<Rc<RefCell<dyn Actor>>> {
        &self.pending_actors
    }

    pub fn get_random(&mut self) -> &mut Random {
        &mut self.random
    }

    pub fn set_updating_actors(&mut self, updating_actors: bool) {
        self.updating_actors = updating_actors;
    }
}
