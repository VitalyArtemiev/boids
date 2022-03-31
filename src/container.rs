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
    pub fn new(pos: Vec2f, num_boids: usize) -> Self {
        assert!(num_boids < CONTAINER_CAPACITY);

        let mut c = Container {
            id: Container::generate_id(),
            ent: BoidVec::random(pos, num_boids),
            state: ContainerState::Hot,
        };

        c.formation_positions.append (&mut c.ent.pos.clone());

        debug_assert!(c.formation_positions.len() == c.ent.pos.len());

        c
    }


    }

    pub fn is_in_bounds(&self, p: Vec2f) -> bool {
        /*let t: Triangle<f64> = [[0.; 2]; 3];
        inside_triangle(t, p.into());*/

        (p - self.center).len() < self.radius
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

    pub fn process_boids1(&mut self, dt: f64) {
        let mut cum_dist_from_dest = 0.0;

        let mut pos1 = Vec2f::default();
        let mut pos2 = Vec2f::default();
        let mut dir = Vec2f::default();

        let len = self.ent.len();

        if let Some(goal) = self.goals.front() {
            match goal {
                Goal::Idle(_) => {
                    self.formation = formation::idle_formation
                }
                Goal::Hold => {
                    //todo: do some idly stuff
                    return
                }
                Goal::Column(_) => {}
                Goal::Front(p1, p2, d) => {
                    pos1 = *p1;
                    pos2 = *p2;
                    dir = *d;
                    self.formation = formation::phalanx_formation
                }

                Goal::Move(p, d) => {
                    pos1 = *p;
                    pos2 = *p;
                    dir = *d;
                }
            }
        }

        let mut center = Vec2f::default();
        let mut max_dist = 0.;

        let rvec = pos2 - pos1;
        let rlen = rvec.len();

        let form_width = (rlen / FORMATION_SPACING).round() as usize;

        for (cur, boid) in self.ent.iter_mut().enumerate() {
            center += *boid.pos;
            max_dist = (*boid.pos - self.center).len().max(max_dist);

            /*let target_offset =
                (self.formation)(cur, form_width).rot_align(rvec) * FORMATION_SPACING;*/

            let target_offset =
                p_f(cur, form_width, rvec.normalise(), dir.normalise())* FORMATION_SPACING;

            let target = pos1 + target_offset;
            let mut d = target - *boid.pos;
            let dist = d.len();
            cum_dist_from_dest += dist;

            if dist < DIST_MARGIN {
                *boid.state = Stationary;
                *boid.vel = Vec2f::default();
                continue;
            } else {
                *boid.state = Marching
            }

            d.clamp(ACC_MAX / 10.);

            for i in cur+1..len {
                //todo: this loop is problematic
                let vec = *boid.pos - *boid.vel;

                let dist = (vec.len() + 0.1).ln() - 4.;

                if vec.man() < DIST_REPEL {
                    d += vec * (500.01 / len as f64) /* * (-dist)*/;
                }
            }

            d.clamp(ACC_MAX);

            *boid.vel += d * dt;
            boid.vel.clamp(VEL_MAX.min(dist));

            *boid.pos += *boid.vel * dt;

            let heading: f64 = f64::atan2(boid.vel.y, boid.vel.x);

            if heading.is_normal() {
                *boid.r = heading;
            }
        }

        center *= 1. / self.ent.len() as f64;

        self.center = center;
        self.radius = max_dist;

        if cum_dist_from_dest < self.ent.len() as f64 * DIST_MARGIN {
            self.goals.pop_front();
            if self.goals.is_empty() {
                self.goals.push_back(Goal::Hold)
            }
        }
    }


    pub fn process_boids(&mut self, dt: f64) {
        let mut cum_dist_from_dest = 0.0;

        let mut pos1 = Vec2f::default();
        let mut pos2 = Vec2f::default();
        let mut dir = Vec2f::default();

        if let Some(goal) = self.goals.front() {
            match goal {
                Goal::Idle(_) => {
                    self.formation = formation::idle_formation
                }
                Goal::Hold => {
                    //todo: do some idly stuff
                    return
                }
                Goal::Column(_) => {}
                Goal::Front(p1, p2, d) => {
                    pos1 = *p1;
                    pos2 = *p2;
                    dir = *d;
                    self.formation = formation::phalanx_formation
                }

                Goal::Move(p, d) => {
                    pos1 = *p;
                    pos2 = *p;
                    dir = *d;
                }
            }
        }

        let mut center = Vec2f::default();
        let mut max_dist = 0.;

        let b = &mut self.ent;

        let rvec = pos2 - pos1;
        let rlen = rvec.len();

        let form_width = (rlen / FORMATION_SPACING).round() as usize;

        for cur in 0..b.len() {
            center += b.pos[cur];
            max_dist = (b.pos[cur] - self.center).len().max(max_dist);

            /*let target_offset =
                (self.formation)(cur, form_width).rot_align(rvec) * FORMATION_SPACING;*/

            let target_offset =
                p_f(cur, form_width, rvec.normalise(), dir.normalise())* FORMATION_SPACING;

            let target = pos1 + target_offset;
            let mut d = target - b.pos[cur];
            let dist = d.len();
            cum_dist_from_dest += dist;

            if dist < DIST_MARGIN {
                b.state[cur] = Stationary;
                b.vel[cur] = Vec2f::default();
                continue;
            } else {
                b.state[cur] = Marching
            }

            d.clamp(ACC_MAX / 10.);

            for i in cur+1..b.len() {
                //todo: this loop is problematic
                let vec = b.pos[cur] - b.vel[i];
                //let dist = (vec.len() + 0.1).ln() - 4.;

                if vec.man() < DIST_REPEL {
                    d += vec * (500.01 / b.len() as f64) /* * (-dist)*/;
                }
            }

            d.clamp(ACC_MAX);

            b.vel[cur] += d * dt;
            b.vel[cur].clamp(VEL_MAX.min(dist));

            b.pos[cur] += b.vel[cur] * dt;

            let heading: f64 = f64::atan2(b.vel[cur].y, b.vel[cur].x);

            if heading.is_normal() {
                b.r[cur] = heading;
            }
        }

        center *= 1. / b.len() as f64;

        self.center = center;
        self.radius = max_dist;

        if cum_dist_from_dest < self.ent.len() as f64 * DIST_MARGIN {
            self.goals.pop_front();
            if self.goals.is_empty() {
                self.goals.push_back(Goal::Hold)
            }
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
