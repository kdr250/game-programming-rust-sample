use std::{cell::RefCell, rc::Rc};

pub trait AIState {
    fn update(&mut self, delta_time: f32) -> Option<String>;

    fn on_enter(&mut self);

    fn on_exit(&mut self);

    fn get_name(&self) -> &String;
}

pub struct AIPatrol {
    name: String,
}

impl AIPatrol {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            name: String::from("Patrol"),
        };

        Rc::new(RefCell::new(this))
    }
}

impl AIState for AIPatrol {
    fn update(&mut self, _delta_time: f32) -> Option<String> {
        println!("Updating {} state", self.get_name());
        let dead = true;
        if dead {
            Some(String::from("Death"))
        } else {
            None
        }
    }

    fn on_enter(&mut self) {
        println!("Entering {} state", self.get_name());
    }

    fn on_exit(&mut self) {
        println!("Exiting {} state", self.get_name());
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct AIDeath {
    name: String,
}

impl AIDeath {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            name: String::from("Death"),
        };

        Rc::new(RefCell::new(this))
    }
}

impl AIState for AIDeath {
    fn update(&mut self, _delta_time: f32) -> Option<String> {
        println!("Updating {} state", self.get_name());
        None
    }

    fn on_enter(&mut self) {
        println!("Entering {} state", self.get_name());
    }

    fn on_exit(&mut self) {
        println!("Exiting {} state", self.get_name());
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct AIAttack {
    name: String,
}

impl AIAttack {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            name: String::from("Attack"),
        };

        Rc::new(RefCell::new(this))
    }
}

impl AIState for AIAttack {
    fn update(&mut self, _delta_time: f32) -> Option<String> {
        println!("Updating {} state", self.get_name());
        None
    }

    fn on_enter(&mut self) {
        println!("Entering {} state", self.get_name());
    }

    fn on_exit(&mut self) {
        println!("Exiting {} state", self.get_name());
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
