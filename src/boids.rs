#![feature(clamp)]
use crate::ops::Vec2f;
use rand::Rng;
use serde::{Deserialize, Serialize};
use soa_derive::StructOfArray;
use crate::boids::BoidState::{Marching, Stationary};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum BoidState {Stationary, Accelerating, Decelerating, Marching, Idle}

impl Default for BoidState {
    fn default() -> Self {
        BoidState::Stationary
    }
}

#[derive(Copy, Clone, StructOfArray, Default, Serialize, Deserialize)]
pub struct Boid {
    pub pos: Vec2f,
    pub vel: Vec2f,
    pub(crate) r: f64,
    pub state: BoidState
}

const SPREAD: f64 = 600.;
const VEL_SPREAD: f64 = 500.;
const ACC_MAX: f64 = 500.;
const VEL_MAX: f64 = 20.;
const DIST_REPEL: f64 = 100.;

impl BoidVec {
    pub fn random(num: usize) -> BoidVec {
        let mut boids = BoidVec::with_capacity(num);
        let mut rng = rand::thread_rng();

        for _ in 0..num {
            boids.push(Boid {
                pos: Vec2f {
                    x: rng.gen::<f64>() * SPREAD,
                    y: rng.gen::<f64>() * SPREAD,
                },
                vel: Vec2f {
                    x: rng.gen::<f64>() * VEL_SPREAD - VEL_SPREAD / 2.,
                    y: rng.gen::<f64>() * VEL_SPREAD - VEL_SPREAD / 2.,
                },
                r: rng.gen::<f64>(),
                state: Default::default()
            });
        }

        boids
    }

    pub fn zeros(num: usize) -> BoidVec {
        let mut boids = BoidVec::with_capacity(num);
        let rng = rand::thread_rng();

        for _ in 0..num {
            boids.push(Boid::default());
        }

        boids
    }

    pub fn upd_position(&mut self, dt: f64, attractor: Vec2f) {
        for cur in 0..self.len() {
            /*match self.state[cur] {
                Stationary => continue,
                Marching => {}
            }*/

            let target = attractor + Vec2f { x: (cur % 5) as f64, y: (cur / 5) as f64 } * 50.;
            let mut d = target - self.pos[cur];
            let dist = d.len();

            if dist < 0.1 {
                self.state[cur] = Stationary;
                self.vel[cur] = Vec2f::default();
                continue
            } else {
                self.state[cur] = Marching
            }

            /*for i in 0..self.len() {
                let vec = self.pos[cur] - self.pos[i];
                let dist = (vec.len() + 0.1).ln() - 4.;

                d += vec * (-dist);
            }*/

            d.clamp(ACC_MAX);

            self.vel[cur] += d * dt;
            self.vel[cur].clamp(VEL_MAX);

            let heading: f64 = f64::atan2(self.vel[cur].y , self.vel[cur].x);

            if heading.is_normal() {
                self.r[cur] = heading;
            }

            self.pos[cur] += self.vel[cur] * dt;
        }
    }
}
