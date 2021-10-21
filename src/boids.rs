#![feature(clamp)]
use soa_derive::StructOfArray;
use rand::Rng;
use crate::ops::Vec2f;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, StructOfArray, Default, Serialize, Deserialize)]
pub struct Boid {
    pub pos: Vec2f,
    pub vel: Vec2f,

    pub(crate) r: f64
}

const SPREAD: f64 = 600.;
const VEL_SPREAD: f64 = 500.;
const ACC_MAX: f64 = 500.;
const VEL_MAX: f64 = 50.;
const DIST_REPEL: f64 = 100.;
const FRICTION: f64 = 0.3;

impl BoidVec {
    pub fn random(num: usize) -> BoidVec {
        let mut boids = BoidVec::with_capacity(num);
        let mut rng = rand::thread_rng();

        for _ in 0..num {
            boids.push(Boid {
                pos: Vec2f{ x: rng.gen::<f64>() * SPREAD, y: rng.gen::<f64>()*SPREAD },
                vel: Vec2f{ x: rng.gen::<f64>()*VEL_SPREAD - VEL_SPREAD/2., y: rng.gen::<f64>()*VEL_SPREAD - VEL_SPREAD/2. } ,
                r: rng.gen::<f64>()});
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
            let mut d = attractor - self.pos[cur];
            let dist = d.len();

            for i in 0..self.len() {
                let vec = self.pos[cur] - self.pos[i];
                let dist = (vec.len() + 0.1).ln() - 5.;

                d +=  vec * ( - dist) ;
            }

            d.clamp(ACC_MAX);

            self.vel[cur] += d * dt * FRICTION ;
            self.vel[cur].clamp(VEL_MAX);

            let heading: f64 = (self.vel[cur].y / self.vel[cur].x).atan();

            if heading.is_normal() {
                self.r[cur] = heading;
            }

            self.pos[cur] += self.vel[cur] * dt;
        }
    }
}