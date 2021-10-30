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
            groups: vec![Container::new(Vec2f::default(),BOID_NUM)],
        }
    }

    pub fn get_ids_at(&self, pos: Vec2f) -> Vec<WorldId> {
        let mut sel = vec![];

        for group in self.groups.iter() {
            if group.is_in_bounds(pos) {
                sel.push(group.id);
                if let Some(b) = group.get_boid_at(pos) {
                    sel.push(b);
                }
            }
        }

        sel
    }

    pub(crate) fn get_ids_in_rect(&self, p0: Vec2f, p1: Vec2f) -> Vec<WorldId> {
        let mut sel = vec![];

        for group in self.groups.iter() {


        }

        sel
    }
}

pub trait Identifiable {
    fn generate_id() -> WorldId;
}
