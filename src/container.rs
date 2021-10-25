use serde::{Serialize, Deserialize};

/// Cold  means calculations only on the whole container
/// Warm allows collision checks
/// Hot allows per-unit operations and info

#[derive(PartialEq, Serialize, Deserialize)]
pub enum ContainerState {
    Cold,
    Warm,
    Hot,
}

impl ContainerState {
    fn minimize(&self) {}

    fn restore(&self) {
        if *self != ContainerState::Cold {}
    }
}

#[derive(Serialize, Deserialize)]
pub enum Goal {
    Idle(Vec2f),
    Column(Vec2f),
    Front(Vec2f)
}

use crate::boids::BoidVec;
use crate::ops::Vec2f;
use crate::player::PlayerAction;
use crate::world::WorldId;

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub id: WorldId,
    pub ent: BoidVec,
    pub goals: Vec<Goal>,

    pub state: ContainerState,
}

impl Container {
    pub fn assign_goals(&mut self, action: PlayerAction) {
        //todo
    }

    fn get_notable(id: usize) -> Option<String> {
        if id == 0 {
            None
        } else {
            Some(String::from("Hello, world!"))
        }
    }

    fn get_info(id: usize) -> String {
        match Container::get_notable(id) {
            None => Container::generate_info(id),
            Some(info) => info,
        }
    }

    fn generate_info(id: usize) -> String {
        todo!()
    }
}
