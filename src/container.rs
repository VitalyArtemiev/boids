use std::sync::atomic::{AtomicUsize, Ordering};
use graphics::math::inside_triangle;
use graphics::types::Triangle;

use crate::boids::BoidState::{Marching, Stationary};
use serde::{Deserialize, Serialize};
use crate::app::CLICK_PRECISION;

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
    Front(Vec2f),
}

use crate::boids::BoidVec;
use crate::ops::Vec2f;
use crate::player::{PlayerAction, PlayerState};
use crate::world::{Identifiable, WORLD_ID, WorldId};

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub id: WorldId,

    pub center: Vec2f,
    pub radius: f64,

    pub ent: BoidVec,

    pub goals: Vec<Goal>,

    pub state: ContainerState,
}

const CONTAINER_CAPACITY: usize = 1024;
const ACC_MAX: f64 = 1000.;
const VEL_MAX: f64 = 100.;
const DIST_REPEL: f64 = 20.;
const FORMATION_SPACING: f64 = 24.;

pub static NUM_CONTAINERS: AtomicUsize = AtomicUsize::new(0);

impl Identifiable for Container {
    fn generate_id() -> WorldId {
        let nc = NUM_CONTAINERS.fetch_add(1, Ordering::Relaxed);
        CONTAINER_CAPACITY * (nc + 1)
    }
}

impl Container {
    pub fn assign_goals(&mut self, action: PlayerAction) {
        //todo
    }

    pub(crate) fn is_in_bounds(&self, p: Vec2f) -> bool {
        /*let t: Triangle<f64> = [[0.; 2]; 3];
        inside_triangle(t, p.into());*/

        return (p - self.center).len() < self.radius;
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

    pub fn get_boid_at(&self, c: Vec2f) -> WorldId {
        let mut i = 0;
        for p in &self.ent.pos {
            if (*p - c).man() < CLICK_PRECISION {
                return self.id + i
            }
        }
        return WORLD_ID
    }

    pub fn process_boids(&mut self, dt: f64, p: &PlayerState) {
        let mut center = Vec2f::default();
        let mut max_dist = 0.;

        let b = &mut self.ent;

        let lvec = p.l2 - p.l1;
        let llen = lvec.len();

        let rvec = p.r2 - p.r1;
        let rlen = rvec.len();

        let form_width = (rlen / FORMATION_SPACING).round() as usize;

        for cur in 0..b.len() {
            center += b.pos[cur];
            max_dist = (b.pos[cur] - self.center).len().max(max_dist);

            let target_offset = Vec2f {
                x: cur.checked_rem(form_width).unwrap_or_default() as f64,
                y: cur.checked_div(form_width).unwrap_or_default() as f64,
            }
            .rot_align(rvec)
                * FORMATION_SPACING;

            let target = p.r1 + target_offset;
            let mut d = target - b.pos[cur];
            let dist = d.len();

            if dist < 1. {
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
    }
}
