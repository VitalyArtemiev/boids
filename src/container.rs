use graphics::math::inside_triangle;
use graphics::types::Triangle;
use rand::Rng;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::app::CLICK_PRECISION;
use crate::boids::BoidState::{Marching, Stationary};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use crate::container::ContainerState::Cold;

/// Cold  means calculations only on the whole container
/// Warm allows collision checks
/// Hot allows per-unit operations and info

#[derive(PartialEq, Serialize, Deserialize)]
pub enum ContainerState {
    Cold,
    Warm,
    Hot,
}

use crate::ops::Vec2f;
use crate::player::{PlayerAction, PlayerState};
use crate::world::{Identifiable, WORLD_ID, WorldId};

pub trait Container {
    fn cur_state(&self) -> ContainerState {
        return Cold
    }

    fn to_cold(&mut self);

    fn to_warm(&mut self);

    fn to_hot(&mut self);
}

#[cfg(test)]
mod tests {
    //use crate::boids::BoidVec;
    use crate::ops::Vec2f;
    use crate::world::Identifiable;
    use quickcheck::quickcheck;
    use std::collections::VecDeque;

    #[test]
    fn container_id_is_correct() {
        /*let c = Container::default();

        assert!(is_container(c.id));

        let id = get_boid_container(c.get_boid_at(c.center).unwrap_or(c.id + 1));

        assert_eq!(id.unwrap(), c.id)*/
    }
}
