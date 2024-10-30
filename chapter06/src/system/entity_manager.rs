use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::{
        actor::{self, Actor, State as ActorState},
        asteroid::Asteroid,
        ship::Ship,
    },
    math::random::Random,
    system::texture_manager::TextureManager,
};

pub struct EntityManager {
    actors: Vec<Rc<RefCell<dyn Actor>>>,
    pending_actors: Vec<Rc<RefCell<dyn Actor>>>,
    updating_actors: bool,
    ship: Option<Rc<RefCell<Ship>>>,
    asteroids: Vec<Rc<RefCell<Asteroid>>>,
    random: Random,
}

impl EntityManager {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            actors: vec![],
            pending_actors: vec![],
            updating_actors: false,
            ship: None,
            asteroids: vec![],
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

        self.asteroids.retain(|asteroid| {
            if *asteroid.borrow().get_state() != ActorState::Dead {
                true
            } else {
                actor::remove_actor(asteroid.clone());
                false
            }
        });
    }

    pub fn load_data(
        this: Rc<RefCell<EntityManager>>,
        texture_manager: Rc<RefCell<TextureManager>>,
    ) {
        let ship = Ship::new(texture_manager.clone(), this.clone());
        this.borrow_mut().ship = Some(ship);

        // Create asteroids
        const NUM_ASTEROIDS: i32 = 20;
        let asteroids: Vec<Rc<RefCell<Asteroid>>> = (0..NUM_ASTEROIDS)
            .into_iter()
            .map(|_| Asteroid::new(texture_manager.clone(), this.clone()))
            .collect();
        this.borrow_mut().set_asteroids(asteroids);
    }

    pub fn get_actors(&self) -> &Vec<Rc<RefCell<dyn Actor>>> {
        &self.actors
    }

    pub fn get_pending_actors(&self) -> &Vec<Rc<RefCell<dyn Actor>>> {
        &self.pending_actors
    }

    pub fn get_asteroids(&self) -> &Vec<Rc<RefCell<Asteroid>>> {
        &self.asteroids
    }

    pub fn set_asteroids(&mut self, asteroids: Vec<Rc<RefCell<Asteroid>>>) {
        self.asteroids = asteroids;
    }

    pub fn get_random(&mut self) -> &mut Random {
        &mut self.random
    }

    pub fn set_updating_actors(&mut self, updating_actors: bool) {
        self.updating_actors = updating_actors;
    }
}
