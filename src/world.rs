use std::any::{Any, TypeId};
use crate::boids::BoidVec;
use crate::container::Container;
use crate::container::ContainerState::Hot;
use crate::container::Goal::Idle;
use crate::ops::Vec2f;
use array2d::Array2D;
use serde::{Deserialize, Serialize};

pub(crate) type WorldId = usize;

///Used to select/deselect everything in the world
pub const WORLD_ID: WorldId = 0;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub groups: Vec<Container>,
    //pub terrain: Array2D<i8>
}

const BOID_NUM: usize = 500;

impl World {
    pub fn single_container() -> World {
        World {
            groups: vec![Container {
                id: Container::generate_id(),
                center: Default::default(),
                radius: 0.0,
                ent: BoidVec::random(BOID_NUM),
                goals: vec![Idle(Vec2f::default())],
                state: Hot,
            }],
        }
    }

    pub fn get_ids_at(&self, pos: Vec2f) -> (WorldId, WorldId) {
        let mut sel = [WORLD_ID; 2];

        let mut i = 0;
        for group in self.groups.iter() {
            if group.is_in_bounds(pos) {
                sel[i] = group.get_boid_at(pos);
                i += 1;
            }
            if i == 2 {
                break
            }
        }

        (sel[0], sel[1])
    }
}

pub trait Identifiable {
    fn generate_id() -> WorldId;
}
