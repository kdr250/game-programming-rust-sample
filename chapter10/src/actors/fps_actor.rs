use core::f32;
use std::{cell::RefCell, rc::Rc};

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    mouse::RelativeMouseState,
};

use crate::{
    collision::aabb::AABB,
    components::{
        audio_component::AudioComponent,
        box_component::BoxComponent,
        component::{Component, State as ComponentState},
        fps_camera::FPSCamera,
        mesh_component::MeshComponent,
        move_component::{DefaultMoveComponent, MoveComponent},
    },
    math::{self, matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        phys_world::PhysWorld, renderer::Renderer, sound_event::SoundEvent,
    },
};

use super::{
    actor::{self, generate_id, Actor, DefaultActor, State},
    ball_actor::BallActor,
};

pub struct FPSActor {
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
    audio_system: Rc<RefCell<AudioSystem>>,
    renderer: Rc<RefCell<Renderer>>,
    phys_world: Rc<RefCell<PhysWorld>>,
    move_component: Option<Rc<RefCell<DefaultMoveComponent>>>,
    camera_component: Option<Rc<RefCell<FPSCamera>>>,
    mesh_component: Option<Rc<RefCell<MeshComponent>>>,
    audio_component: Option<Rc<RefCell<AudioComponent>>>,
    box_component: Option<Rc<RefCell<BoxComponent>>>,
    fps_model: Option<Rc<RefCell<DefaultActor>>>,
    foot_step: Option<Rc<RefCell<SoundEvent>>>,
    last_foot_step: f32,
}

impl FPSActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
        audio_system: Rc<RefCell<AudioSystem>>,
        renderer: Rc<RefCell<Renderer>>,
        phys_world: Rc<RefCell<PhysWorld>>,
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
            audio_system: audio_system.clone(),
            renderer: renderer.clone(),
            phys_world: phys_world.clone(),
            move_component: None,
            camera_component: None,
            mesh_component: None,
            audio_component: None,
            box_component: None,
            fps_model: None,
            foot_step: None,
            last_foot_step: 0.0,
        };

        let result = Rc::new(RefCell::new(this));

        let move_component = DefaultMoveComponent::new(result.clone());
        result.borrow_mut().move_component = Some(move_component);

        let audio_component = AudioComponent::new(result.clone(), audio_system.clone());
        let sound_event = audio_component
            .borrow_mut()
            .play_event("event:/Footstep", &result.borrow().get_world_transform());
        sound_event.borrow_mut().set_paused(true);
        result.borrow_mut().audio_component = Some(audio_component);
        result.borrow_mut().foot_step = Some(sound_event);

        let fps_camera = FPSCamera::new(result.clone(), renderer, audio_system);
        result.borrow_mut().camera_component = Some(fps_camera);

        let fps_model = DefaultActor::new(asset_manager.clone(), entity_manager.clone());
        fps_model.borrow_mut().set_scale(0.75);

        let mesh_component = MeshComponent::new(fps_model.clone());
        let mesh = asset_manager.borrow_mut().get_mesh("Rifle.gpmesh");
        mesh_component.borrow_mut().set_mesh(mesh);

        result.borrow_mut().fps_model = Some(fps_model);
        result.borrow_mut().mesh_component = Some(mesh_component);

        let box_component = BoxComponent::new(result.clone(), phys_world);
        let collision = AABB::new(
            Vector3::new(-25.0, -25.0, -87.5),
            Vector3::new(25.0, 25.0, 87.5),
        );
        box_component.borrow_mut().set_object_box(collision);
        box_component.borrow_mut().set_should_rotate(false);
        result.borrow_mut().box_component = Some(box_component);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn set_foot_step_surface(&mut self, value: f32) {
        // Pause here because the way I setup the parameter in FMOD
        // changing it will play a footstep
        let foot_step = self.foot_step.clone().unwrap();
        foot_step.borrow_mut().set_paused(true);
        foot_step.borrow_mut().set_parameter("Surface", value);
    }

    pub fn fix_collision(&mut self) {
        // Need to recompute my world transform to update world box
        self.compute_world_transform();

        let box_component = self.box_component.clone().unwrap();
        let mut borrowed_box_component = box_component.borrow_mut();
        let player_box = borrowed_box_component.get_world_box();
        let position = self.get_position();
        let mut new_positions = vec![];

        let planes = self.entity_manager.borrow().get_planes().clone();
        for plane in planes {
            // Do we collide with this PlaneActor ?
            let borrowed_plane = plane.borrow();
            let plane_box = borrowed_plane.get_box().borrow().get_world_box().clone();
            if AABB::intersect(&player_box, &plane_box) {
                // Calculate all our differences
                let dx1 = plane_box.max.x - player_box.min.x;
                let dx2 = plane_box.min.x - player_box.max.x;
                let dy1 = plane_box.max.y - player_box.min.y;
                let dy2 = plane_box.min.y - player_box.max.y;
                let dz1 = plane_box.max.z - player_box.min.z;
                let dz2 = plane_box.min.z - player_box.max.z;

                // Set dx to whichever of dx1/dx2 dy1/dy2 dz1/dz2 have a lower abs
                let dx = if dx1.abs() < dx2.abs() { dx1 } else { dx2 };
                let dy = if dy1.abs() < dy2.abs() { dy1 } else { dy2 };
                let dz = if dz1.abs() < dz2.abs() { dz1 } else { dz2 };

                // Whichever is closest, adjust x/y position
                let mut new_position = position.clone();
                if dx.abs() <= dy.abs() && dx.abs() <= dz.abs() {
                    new_position.x += dx;
                } else if dy.abs() <= dx.abs() && dy.abs() <= dz.abs() {
                    new_position.y += dy;
                } else {
                    new_position.z += dz;
                }
                new_positions.push(new_position);
            }
        }

        for position in new_positions {
            // Need to set position and update box component
            self.set_position(position);
            let owner_info = (
                self.get_position().clone(),
                self.get_scale(),
                self.get_rotation().clone(),
            );
            borrowed_box_component.on_update_world_transform(&owner_info);
        }
    }

    pub fn shoot(&mut self) {
        // Get start point (in center of screen on near plane)
        let mut screen_point = Vector3::ZERO;
        let start = self.renderer.borrow().unproject(screen_point.clone());
        // Get end point (in center of screen, between near and far)
        screen_point.z = 0.9;
        let end = self.renderer.borrow().unproject(screen_point.clone());
        // Get direction vector
        let mut dir = end.clone() - start.clone();
        dir.normalize_mut();
        // Spawn a ball
        let ball = BallActor::new(
            self.asset_manager.clone(),
            self.entity_manager.clone(),
            self.phys_world.clone(),
            self.get_id(),
            self.audio_system.clone(),
        );
        ball.borrow_mut().set_position(start + dir.clone() * 20.0);
        // Rotate the ball to face new direction
        ball.borrow_mut().rotate_to_new_forward(dir);
        // Play shooting sound
        self.audio_component
            .as_ref()
            .unwrap()
            .borrow_mut()
            .play_event("event:/Shot", &self.get_world_transform());
    }
}

impl Actor for FPSActor {
    fn update_actor(&mut self, delta_time: f32) {
        self.fix_collision();

        // Play the footstep if we're moving and haven't recently
        self.last_foot_step -= delta_time;
        if !math::basic::near_zero(
            self.move_component
                .clone()
                .unwrap()
                .borrow()
                .get_forward_speed(),
            0.001,
        ) && self.last_foot_step <= 0.0
        {
            let foot_step = self.foot_step.clone().unwrap();
            foot_step.borrow_mut().set_paused(false);
            foot_step.borrow_mut().restart();
            self.last_foot_step = 0.5;
        }

        // Update position of FPS model relative to actor position
        let model_offset = Vector3::new(10.0, 10.0, -10.0);
        let mut model_position = self.get_position().clone();
        model_position += self.get_forward() * model_offset.x;
        model_position += self.get_right() * model_offset.y;
        model_position.z += model_offset.z;

        let fps_model = self.fps_model.as_ref().unwrap();
        fps_model.borrow_mut().set_position(model_position);

        // Initialize rotation to actor rotation
        let mut q = self.get_rotation().clone();

        // Rotate by pitch from camera
        let camera_component = self.camera_component.as_ref().unwrap();
        q = Quaternion::concatenate(
            &q,
            &Quaternion::from_axis_angle(&self.get_right(), camera_component.borrow().get_pitch()),
        );
        fps_model.borrow_mut().set_rotation(q);
    }

    fn actor_input(&mut self, key_state: &KeyboardState, mouse_state: &RelativeMouseState) {
        let mut forward_speed = 0.0;
        let mut strafe_speed = 0.0;

        if key_state.is_scancode_pressed(Scancode::W) {
            forward_speed += 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::S) {
            forward_speed -= 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::A) {
            strafe_speed -= 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::D) {
            strafe_speed += 400.0;
        }

        let move_component = self.move_component.clone().unwrap();
        move_component.borrow_mut().set_forward_speed(forward_speed);
        move_component.borrow_mut().set_strafe_speed(strafe_speed);

        // Mouse movement
        // Get relative movement from SDL
        let x = mouse_state.x();
        let y = mouse_state.y();

        // Assume mouse movement is usually between -500 and +500
        let max_mouse_speed = 500.0;

        // Rotation/sec at maximum speed
        let max_angular_speed = f32::consts::PI * 8.0;

        let mut angular_speed = 0.0;
        if x != 0 {
            // Convert to ~[-1.0, 1.0]
            angular_speed = x as f32 / max_mouse_speed;
            // Multiply by rotation/sec
            angular_speed *= max_angular_speed;
        }
        move_component.borrow_mut().set_angular_speed(angular_speed);

        // Compute pitch
        let max_pitch_speed = f32::consts::PI * 8.0;
        let mut pitch_speed = 0.0;
        if y != 0 {
            // Convert to [-1.0, 1.0]
            pitch_speed = y as f32 / max_mouse_speed;
            pitch_speed *= max_pitch_speed;
        }

        let camera_component = self.camera_component.clone().unwrap();
        camera_component.borrow_mut().set_pitch_speed(pitch_speed);
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}
