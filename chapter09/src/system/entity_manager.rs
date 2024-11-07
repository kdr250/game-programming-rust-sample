use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::{
        actor::{self, Actor, DefaultActor, State as ActorState},
        camera_actor::CameraActor,
        follow_actor::FollowActor,
        fps_actor::FPSActor,
        orbit_actor::{self, OrbitActor},
        plane_actor::PlaneActor,
        spline_actor::SplineActor,
    },
    components::{
        audio_component::AudioComponent,
        mesh_component::MeshComponent,
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::{quaternion::Quaternion, random::Random, vector3::Vector3},
    system::{asset_manager::AssetManager, renderer::Renderer},
};

use super::audio_system::AudioSystem;

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
        audio_system: Rc<RefCell<AudioSystem>>,
    ) -> (
        Rc<RefCell<FPSActor>>,
        Rc<RefCell<FollowActor>>,
        Rc<RefCell<OrbitActor>>,
        Rc<RefCell<SplineActor>>,
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

        // Setup floor
        let start = -1250.0;
        let size = 250.0;
        for i in 0..10 {
            for j in 0..10 {
                let p = PlaneActor::new(asset_manager.clone(), this.clone());
                p.borrow_mut().set_position(Vector3::new(
                    start + i as f32 * size,
                    start + j as f32 * size,
                    -100.0,
                ));
            }
        }

        // Left/right walls
        let q = Quaternion::from_axis_angle(&Vector3::UNIT_X, std::f32::consts::FRAC_PI_2);
        for i in 0..10 {
            let p = PlaneActor::new(asset_manager.clone(), this.clone());
            p.borrow_mut()
                .set_position(Vector3::new(start + i as f32 * size, start - size, 0.0));
            p.borrow_mut().set_rotation(q.clone());

            let p = PlaneActor::new(asset_manager.clone(), this.clone());
            p.borrow_mut()
                .set_position(Vector3::new(start + i as f32 * size, -start + size, 0.0));
            p.borrow_mut().set_rotation(q.clone());
        }

        // Forward/back walls
        let q = Quaternion::concatenate(
            &q,
            &Quaternion::from_axis_angle(&Vector3::UNIT_Z, std::f32::consts::FRAC_PI_2),
        );
        for i in 0..10 {
            let p = PlaneActor::new(asset_manager.clone(), this.clone());
            p.borrow_mut()
                .set_position(Vector3::new(start - size, start + i as f32 * size, 0.0));
            p.borrow_mut().set_rotation(q.clone());

            let p = PlaneActor::new(asset_manager.clone(), this.clone());
            p.borrow_mut()
                .set_position(Vector3::new(-start + size, start + i as f32 * size, 0.0));
            p.borrow_mut().set_rotation(q.clone());
        }

        // Setup lights
        {
            let mut borrowed_renderer = renderer.borrow_mut();
            borrowed_renderer.set_ambient_light(Vector3::new(0.2, 0.2, 0.2));
            let directional_light = borrowed_renderer.get_directional_light_mut();
            directional_light.direction = Vector3::new(0.0, -0.707, -0.707);
            directional_light.diffuse_color = Vector3::new(0.78, 0.88, 1.0);
            directional_light.spec_color = Vector3::new(0.8, 0.8, 0.8);
        }

        // UI elements
        let ui = DefaultActor::new(asset_manager.clone(), this.clone());
        ui.borrow_mut()
            .set_position(Vector3::new(-350.0, -350.0, 0.0));
        let sprite_component = DefaultSpriteComponent::new(ui.clone(), 100);
        let texture = asset_manager.borrow_mut().get_texture("HealthBar.png");
        sprite_component.borrow_mut().set_texture(texture);

        let ui = DefaultActor::new(asset_manager.clone(), this.clone());
        ui.borrow_mut()
            .set_position(Vector3::new(375.0, -275.0, 0.0));
        ui.borrow_mut().set_scale(0.75);
        let sprite_component = DefaultSpriteComponent::new(ui.clone(), 100);
        let texture = asset_manager.borrow_mut().get_texture("Radar.png");
        sprite_component.borrow_mut().set_texture(texture);

        // Create spheres with audio components playing different sounds
        let m = DefaultActor::new(asset_manager.clone(), this.clone());
        m.borrow_mut().set_position(Vector3::new(500.0, -75.0, 0.0));
        m.borrow_mut().set_scale(1.0);
        let mc = MeshComponent::new(m.clone());
        let mesh = asset_manager.borrow_mut().get_mesh("Sphere.gpmesh");
        mc.borrow_mut().set_mesh(mesh);
        let ac = AudioComponent::new(m, audio_system.clone());
        ac.borrow_mut().play_event("event:/FireLoop");

        // Different camera actors
        let fps_actor = FPSActor::new(
            asset_manager.clone(),
            this.clone(),
            audio_system.clone(),
            renderer.clone(),
        );
        let follow_actor = FollowActor::new(
            asset_manager.clone(),
            this.clone(),
            audio_system.clone(),
            renderer.clone(),
        );
        let orbit_actor = OrbitActor::new(
            asset_manager.clone(),
            this.clone(),
            audio_system.clone(),
            renderer.clone(),
        );
        let spline_actor = SplineActor::new(asset_manager, this, audio_system, renderer);

        (fps_actor, follow_actor, orbit_actor, spline_actor)
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
