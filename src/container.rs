use graphics::math::inside_triangle;
use graphics::types::Triangle;
use rand::Rng;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::app::CLICK_PRECISION;
use crate::boids::BoidState::{Marching, Stationary};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

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
    Hold,
    Move(Vec2f, Vec2f),
    Column(Vec2f),
    Front(Vec2f, Vec2f, Vec2f),
}

use crate::boids::BoidVec;
use crate::container::Goal::Idle;
use crate::formation;
use crate::formation::{FormationFunction, p_f};
use crate::ops::Vec2f;
use crate::player::PlayerAction::FormUp;
use crate::player::{PlayerAction, PlayerState};
use crate::world::{Identifiable, WORLD_ID, WorldId};

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub id: WorldId,

    pub selected: bool,

    pub center: Vec2f,
    pub direction: Vec2f,
    pub radius: f64,

    pub ent: BoidVec,
    ///.first is next goal
    pub goals: VecDeque<Goal>,

    pub state: ContainerState,

    #[serde(skip)]
    #[serde(default = "crate::formation::crutch")]
    pub formation: FormationFunction,
}

const CONTAINER_CAPACITY: usize = 1024;
const ACC_MAX: f64 = 1000.;
const VEL_MAX: f64 = 100.;
const DIST_REPEL: f64 = 20.;
const DIST_MARGIN: f64 = 1.;
pub const FORMATION_SPACING: f64 = 24.;
const COLUMN_WIDTH: i32 = 4;

pub static NUM_CONTAINERS: AtomicUsize = AtomicUsize::new(0);

//const STRUCTURE_BITS: usize = 0b0011_1111_1111_0000_0000_0000_0000_0000; this is more robust
//const CONTAINER_BITS: usize = 0b0000_0000_0000_1111_1111_1100_0000_0000;

impl Identifiable for Container {
    fn generate_id() -> WorldId {
        let nc = NUM_CONTAINERS.fetch_add(1, Ordering::Relaxed);
        CONTAINER_CAPACITY * (nc + 1)
    }
}

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

        Container {
            id: Container::generate_id(),
            selected: false,
            center: pos,
            direction: Vec2f{x: 1.,y: 0.},
            radius: 0.0,
            ent: BoidVec::random(pos, num_boids),
            goals: VecDeque::from([Idle(pos)]),
            state: ContainerState::Hot,
            formation: formation::default_formation,
        }
    }

    pub fn assign_goals(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::None => {}
            PlayerAction::Move(pos, dir) => {
                self.goals.clear();
                self.goals.push_back(Goal::Move(pos, dir.unwrap_or(self.direction)))
            }
            PlayerAction::AddMove(pos, dir) => {
                self.goals.push_back(Goal::Move(pos, dir.unwrap_or(self.direction)))
            }
            PlayerAction::FormUp(pos1, pos2) => {
                let center_front = (pos2 - pos1) * 0.5;
                let dir = center_front - self.center;

                self.goals.clear();
                self.goals.push_back(Goal::Front(pos2, pos1, dir))
            }
            PlayerAction::AddFormUp(pos1, pos2) => {
                let center_front = (pos2 - pos1) * 0.5;
                let dir = center_front - self.center;

                self.goals.push_back(Goal::Front(pos2, pos1, dir))
            }
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

            for i in 0..b.len() {
                //todo: this loop is problematic
                let vec = b.pos[cur] - b.pos[i];
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
