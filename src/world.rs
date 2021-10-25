use array2d::Array2D;
use crate::container::Container;
use serde::{Serialize, Deserialize};
use crate::boids::BoidVec;
use crate::container::ContainerState::Hot;
use crate::container::Goal::Idle;
use crate::ops::Vec2f;

pub(crate) type WorldId = usize;

///Used to select/deselect everything in the world
pub const WORLD_ID: WorldId = 0;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub groups: Vec<Container>,
    //pub terrain: Array2D<i8>
}

impl World {
    pub fn single_container(boids: BoidVec) -> World{
        World {
            groups: vec!(Container {id: 100, ent: boids, goals: vec![Idle(Vec2f::default())], state: Hot })
        }
    }
}