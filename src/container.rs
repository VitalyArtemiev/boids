/// Cold  means calculations only on the whole container
/// Warm allows collision checks
/// Hot allows per-unit operations and info

#[derive(PartialEq)]
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

pub enum Goal {
    Idle(Vec2f),
    Column(Vec2f),
    Front(Vec2f)
}

use crate::boids::BoidVec;
use crate::ops::Vec2f;

pub struct Container {
    pub ent: BoidVec,
    pub goals: Vec<Goal>,

    pub state: ContainerState,
}

impl Container {
    fn get_notable(id: usize) -> Option<String> {
        if id == 0 {
            return None;
        } else {
            return Some(String::from("Hello, world!"));
        }
    }

    fn get_info(id: usize) -> String {
        match Container::get_notable(id) {
            None => return Container::generate_info(id),
            Some(info) => return info,
        }
    }

    fn generate_info(id: usize) -> String {
        todo!()
    }
}
