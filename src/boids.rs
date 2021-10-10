#![feature(clamp)]
use soa_derive::StructOfArray;
use rand::Rng;
use graphics::math::Vec2d;
use crate::ops::Vec2f;

#[derive(Copy, Clone, StructOfArray)]
pub struct Boid {
    pub pos: Vec2d,
    pub vel: Vec2d,

    pub(crate) x: f64,
    pub(crate) y: f64,

    pub(crate) xv: f64,
    pub(crate) yv: f64,

    pub(crate) r: f64
}

impl Default for Boid {
    fn default() -> Boid {
        Boid { pos: Vec2d::default(), vel: Vec2d::default(), x: 0., y: 0., xv: 0., yv: 0., r: 0. }
    }
}

const SPREAD: f64 = 600.;
const VEL_SPREAD: f64 = 500.;
const VEL_MAX: f64 = 500.;
const DIST_REPEL: f64 = 100.;

struct Vec22(f64, f64);

impl BoidVec {
    pub fn random(num: usize) -> BoidVec {
        let mut boids = BoidVec::with_capacity(num);
        let mut rng = rand::thread_rng();

        for _ in 0..num {
            boids.push(Boid {
                pos: [rng.gen::<f64>()*SPREAD; 2],
                vel: [rng.gen::<f64>()*VEL_SPREAD - VEL_SPREAD/2.; 2],
                x: rng.gen::<f64>()*SPREAD, y: rng.gen::<f64>()*SPREAD,
                xv: rng.gen::<f64>()*VEL_SPREAD - VEL_SPREAD/2., yv: rng.gen::<f64>()*VEL_SPREAD - VEL_SPREAD/2., 
                r: rng.gen::<f64>()});
        }

        boids
    }

    pub fn zeros(num: usize) -> BoidVec {
        let mut boids = BoidVec::with_capacity(num);
        let rng = rand::thread_rng();

        for _ in 0..num {
            boids.push(Boid {
                pos: [0.; 2],
                vel: [0.; 2],
                x: 0.1, y: -0.1,
                xv: 0., yv: 0.,
                r: 0.});
        }

        boids
    }

    pub fn upd_position(&mut self, dt:f64, attractor: Vec2f) {
        for cur in 0..self.len() {
            let mut dx = attractor.x - self.x[cur];
            let mut dy = attractor.y - self.y[cur];

            //let mut d = ATTRACTOR - self.pos[cur];

            let v = Vec2f {x: 0., y: 0.  };
            let b = Vec2f {x: 1., y: 2.  };
            let c = v + b;

            for i in 0..self.len() {
                if (self.x[cur] - self.x[i]).abs() < DIST_REPEL {
                    dx -= (self.x[i] - self.x[cur]) * 5.;
                }
                if (self.y[cur] - self.y[i]).abs() < DIST_REPEL {
                    dy -= (self.y[i] - self.y[cur]) * 5.;
                }
            }
            
            //self.r[cur] = (dx/dy).atan();

            //self.xv[cur] = (self.xv[cur] + (xv - self.xv[cur])/8.).clamp(-VEL_MAX, VEL_MAX);
            //self.yv[cur] = (self.yv[cur] + (yv - self.yv[cur])/8.).clamp( -VEL_MAX, VEL_MAX);

            self.xv[cur] = (self.xv[cur] + dx * dt).clamp(-VEL_MAX, VEL_MAX);
            self.yv[cur] = (self.yv[cur] + dy * dt).clamp( -VEL_MAX, VEL_MAX);
 
            self.r[cur] = (self.xv[cur]/self.yv[cur]).atan();

            self.x[cur] += self.xv[cur] * dt;
            self.y[cur] += self.yv[cur] * dt;
        }
    }
}