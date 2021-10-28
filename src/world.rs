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

impl World {
    pub fn single_container(boids: BoidVec) -> World {
        World {
            groups: vec![Container {
                id: 100,
                center: Default::default(),
                ent: boids,
                foo: 0,
                goals: vec![Idle(Vec2f::default())],
                state: Hot,
            }],
        }
    }

    pub fn get_ids_at(&self, pos: Vec2f) -> (Option<WorldId>, Option<WorldId>) {
        for group in self.groups.iter() {
            if group.is_in_bounds(pos) {}
        }

        (None, None)
    }
}
