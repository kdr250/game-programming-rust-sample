use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::{
        actor::{self, Actor, DefaultActor, State as ActorState},
        fps_actor::FPSActor,
        plane_actor::PlaneActor,
        target_actor::TargetActor,
    },
    components::sprite_component::{DefaultSpriteComponent, SpriteComponent},
    math::{quaternion::Quaternion, random::Random, vector3::Vector3},
    system::{asset_manager::AssetManager, renderer::Renderer},
};

use super::{audio_system::AudioSystem, phys_world::PhysWorld};

pub struct EntityManager {
    actors: Vec<Rc<RefCell<dyn Actor>>>,
    pending_actors: Vec<Rc<RefCell<dyn Actor>>>,
    updating_actors: bool,
    fps_actor: Option<Rc<RefCell<FPSActor>>>,
    planes: Vec<Rc<RefCell<PlaneActor>>>,
    random: Random,
}

impl EntityManager {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            actors: vec![],
            pending_actors: vec![],
            updating_actors: false,
            fps_actor: None,
            planes: vec![],
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
        phys_world: Rc<RefCell<PhysWorld>>,
    ) -> Rc<RefCell<FPSActor>> {
        let mut planes = vec![];

        // Setup floor
        let start = -1250.0;
        let size = 250.0;
        for i in 0..10 {
            for j in 0..10 {
                let p = PlaneActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
                p.borrow_mut().set_position(Vector3::new(
                    start + i as f32 * size,
                    start + j as f32 * size,
                    -100.0,
                ));
                planes.push(p);
            }
        }

        // Left/right walls
        let q = Quaternion::from_axis_angle(&Vector3::UNIT_X, std::f32::consts::FRAC_PI_2);
        for i in 0..10 {
            let p = PlaneActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
            p.borrow_mut()
                .set_position(Vector3::new(start + i as f32 * size, start - size, 0.0));
            p.borrow_mut().set_rotation(q.clone());

            let p = PlaneActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
            p.borrow_mut()
                .set_position(Vector3::new(start + i as f32 * size, -start + size, 0.0));
            p.borrow_mut().set_rotation(q.clone());
            planes.push(p);
        }

        // Forward/back walls
        let q = Quaternion::concatenate(
            &q,
            &Quaternion::from_axis_angle(&Vector3::UNIT_Z, std::f32::consts::FRAC_PI_2),
        );
        for i in 0..10 {
            let p = PlaneActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
            p.borrow_mut()
                .set_position(Vector3::new(start - size, start + i as f32 * size, 0.0));
            p.borrow_mut().set_rotation(q.clone());

            let p = PlaneActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
            p.borrow_mut()
                .set_position(Vector3::new(-start + size, start + i as f32 * size, 0.0));
            p.borrow_mut().set_rotation(q.clone());
            planes.push(p);
        }

        this.borrow_mut().planes = planes;

        // Camera actor
        let fps_actor = FPSActor::new(
            asset_manager.clone(),
            this.clone(),
            audio_system.clone(),
            renderer.clone(),
            phys_world.clone(),
        );
        this.borrow_mut().fps_actor = Some(fps_actor.clone());

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

        // Create target actors
        let t = TargetActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
        t.borrow_mut()
            .set_position(Vector3::new(1450.0, 0.0, 100.0));
        let t = TargetActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
        t.borrow_mut()
            .set_position(Vector3::new(1450.0, 0.0, 400.0));
        let t = TargetActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
        t.borrow_mut()
            .set_position(Vector3::new(1450.0, -500.0, 200.0));
        let t = TargetActor::new(asset_manager.clone(), this.clone(), phys_world.clone());
        t.borrow_mut()
            .set_position(Vector3::new(1450.0, 500.0, 200.0));

        fps_actor
    }

    pub fn get_actors(&self) -> &Vec<Rc<RefCell<dyn Actor>>> {
        &self.actors
    }

    pub fn get_pending_actors(&self) -> &Vec<Rc<RefCell<dyn Actor>>> {
        &self.pending_actors
    }

    pub fn get_planes(&self) -> &Vec<Rc<RefCell<PlaneActor>>> {
        &self.planes
    }

    pub fn get_random(&mut self) -> &mut Random {
        &mut self.random
    }

    pub fn set_updating_actors(&mut self, updating_actors: bool) {
        self.updating_actors = updating_actors;
    }
}
