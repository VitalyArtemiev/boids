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

pub trait ContainerTrait {
    fn cur_state(&self) -> ContainerState {
        return Cold
    }

    fn to_cold(&mut self) {

    }

    fn to_warm(&mut self) {

    }

    fn to_hot(&mut self) {

    }
}

const ACC_MAX: f64 = 1000.;
const VEL_MAX: f64 = 100.;
const DIST_REPEL: f64 = 20.;
const DIST_MARGIN: f64 = 1.;
pub const FORMATION_SPACING: f64 = 24.;
const COLUMN_WIDTH: i32 = 4;


//const STRUCTURE_BITS: usize = 0b0011_1111_1111_0000_0000_0000_0000_0000; this is more robust
//const CONTAINER_BITS: usize = 0b0000_0000_0000_1111_1111_1100_0000_0000;



pub fn is_container(id: WorldId) -> bool {
    id % CONTAINER_CAPACITY == 0
}

pub fn is_boid_of_container(boid: WorldId, container: WorldId) -> bool {
    boid - boid % CONTAINER_CAPACITY == container
}

pub fn get_boid_container(id: WorldId) -> Option<WorldId> {
    //boid & CONTAINER_BITS shr 10;
    let result = id - id % CONTAINER_CAPACITY;

    if result < CONTAINER_CAPACITY {
        None
    } else {
        Some(result)
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::new(Vec2f::default(), 20)
    }
}

impl Container {
    


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

    //todo: bench normal loop; should be zero-cost
    pub fn get_boid_at(&self, c: Vec2f) -> Option<WorldId> {
        for (i, p) in self.ent.pos.iter().enumerate() {
            if (*p - c).man() < CLICK_PRECISION {
                return Some(self.id + i + 1);
            }
        }
        None
    }

    pub fn calculate_formation(&mut self, flen: f64) {
        let form_width = (flen / FORMATION_SPACING).round() as usize;

        for (i, pos) in self.formation_positions.iter_mut().enumerate() {
            *pos = (self.formation)(i, form_width);
        }
    }

    pub fn p_b(&mut self, dt: f64) {
        //calc cum_dist
        let mut cum_dist = 0.0;

        //get formation, rotate according to heading
        //after forming, rotate formation only within 1 quadrant, other dirs handled by rotating individual boids

        //let iter = ;

        let mut slice = self.ent.as_mut_slice();

        let slen = slice.len();


        for (i, boid) in slice.iter_mut().enumerate() {
            let d = self.formation_positions[i] - *boid.pos;

            let dist = d.len();
            cum_dist += dist;

            //let slice = self.ent.slice_mut(i+1..self.ent.len());

            /*for j in i+1..slen {
                slice.pos[j];
            }*/

            *boid.vel += d * dt;
            boid.vel.clamp(VEL_MAX.min(dist));

            *boid.pos += *boid.vel * dt;

            let heading: f64 = f64::atan2(boid.vel.y, boid.vel.x);

            if heading.is_normal() {
                *boid.r = heading;
            }
        }
        //project along a heading vector, maybe along a spline
        //the more the curve, the slower the step? or adjust speed manually
        //kinematic step
        //do collision detection, from inside out?
        if cum_dist < self.ent.len() as f64 * DIST_MARGIN {
            self.goals.pop_front();
            if self.goals.is_empty() {
                self.goals.push_back(Goal::Hold)
            }
        }
    }

#[cfg(test)]
mod tests {
    use crate::boids::BoidVec;
    use crate::container::{Container, ContainerState, get_boid_container, is_container};
    use crate::ops::Vec2f;
    use crate::world::Identifiable;
    use quickcheck::quickcheck;
    use std::collections::VecDeque;

    #[test]
    fn container_id_is_correct() {
        let c = Container::default();

        assert!(is_container(c.id));

        let id = get_boid_container(c.get_boid_at(c.center).unwrap_or(c.id + 1));

        assert_eq!(id.unwrap(), c.id)
    }
}
