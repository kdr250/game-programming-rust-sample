use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{audio_system::AudioSystem, renderer::Renderer},
};

use super::{
    camera_component::{self, CameraComponent},
    component::{self, generate_id, Component, State},
};

pub struct Spline {
    // Control points for spline
    // (Requires n+2 points where n is number
    // of points in segment)
    pub control_points: Vec<Vector3>,
}

impl Spline {
    pub fn new() -> Self {
        Self {
            control_points: vec![Vector3::ZERO; 4],
        }
    }

    /// Given spline segment where startIndex = P1,
    /// compute position based on t value
    pub fn compute(&mut self, start_index: usize, t: f32) -> Vector3 {
        // Check if startIdx is out of bounds
        if start_index >= self.control_points.len() {
            return self.control_points.last().unwrap().clone();
        }
        if start_index == 0 {
            return self.control_points[start_index].clone();
        }
        if start_index + 2 >= self.control_points.len() {
            return self.control_points[start_index].clone();
        }

        // Get p0 through p3
        let p0 = self.control_points[start_index - 1].clone();
        let p1 = self.control_points[start_index].clone();
        let p2 = self.control_points[start_index + 1].clone();
        let p3 = self.control_points[start_index + 2].clone();

        // Compute position according to Catmull-Rom equation
        let position = ((p1.clone() * 2.0)
            + (p0.clone() * -1.0 + p2.clone()) * t
            + (p0.clone() * 2.0 - p1.clone() * 5.0 + p2.clone() * 4.0 - p3.clone()) * t * t
            + (p0.clone() * -1.0 + p1.clone() * 3.0 - p2.clone() * 3.0 + p3.clone()) * t * t * t)
            * 0.5;

        position
    }

    pub fn get_num_points(&self) -> usize {
        self.control_points.len()
    }
}

pub struct SplineCamera {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    renderer: Rc<RefCell<Renderer>>,
    audio_system: Rc<RefCell<AudioSystem>>,
    path: Spline,
    index: usize,
    t: f32,
    speed: f32,
    paused: bool,
}

impl SplineCamera {
    pub fn new(
        owner: Rc<RefCell<dyn Actor>>,
        renderer: Rc<RefCell<Renderer>>,
        audio_system: Rc<RefCell<AudioSystem>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 200,
            state: State::Active,
            renderer,
            audio_system,
            path: Spline::new(),
            index: 1,
            t: 0.0,
            speed: 0.5,
            paused: true,
        };
        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        result
    }

    pub fn restart(&mut self) {
        self.index = 1;
        self.t = 0.0;
        self.paused = false;
    }

    pub fn set_spline(&mut self, path: Spline) {
        self.path = path;
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}

impl CameraComponent for SplineCamera {
    camera_component::impl_getters! {}
}

impl Component for SplineCamera {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        // Update t value
        if !self.paused {
            self.t += self.speed * delta_time;

            // Advance to the next control point if needed.
            // This assumes speed isn't so fast that you jump past
            // multiple control points in one frame.
            if self.t >= 1.0 {
                if self.index < self.path.get_num_points() - 3 {
                    self.index += 1;
                    self.t = self.t - 1.0;
                } else {
                    // Path's done, so pause
                    self.paused = true;
                }
            }
        }

        // Camera position is the spline at the current t/index
        let camera_position = self.path.compute(self.index, self.t);
        // Target point is just a small delta ahead on the spline
        let target = self.path.compute(self.index, self.t + 0.01);
        // Assume spline doesn't flip upside-down
        let up = Vector3::UNIT_Z;

        let view = Matrix4::create_look_at(&camera_position, &target, &up);
        self.set_view_matrix(view);

        (None, None)
    }

    component::impl_getters_setters! {}
}
