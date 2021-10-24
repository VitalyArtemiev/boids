#![feature(clamp)]
use crate::app::PlayerState;
use crate::boids::BoidState::{Marching, Stationary};
use crate::ops::Vec2f;
use rand::Rng;
use serde::{Deserialize, Serialize};
use soa_derive::StructOfArray;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum BoidState {
    Stationary,
    Accelerating,
    Decelerating,
    Marching,
    Idle,
}

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
    pub state: BoidState,
    pub color: [f32; 4]
}

const SPREAD: f64 = 600.;
const VEL_SPREAD: f64 = 500.;
const ACC_MAX: f64 = 1000.;
const VEL_MAX: f64 = 100.;
const DIST_REPEL: f64 = 100.;
const FORMATION_SPACING: f64 = 24.;

impl BoidVec {
    pub fn random(num: usize) -> BoidVec {
        let mut boids = BoidVec::with_capacity(num);
        let mut rng = rand::thread_rng();

        for i in 0..num {
            let c = i as f32 / num as f32;
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
                state: Default::default(),
                color: [c, c, c, 1.2 - c]
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

    pub fn upd_position(&mut self, dt: f64, p: &PlayerState) {
        let lvec = p.l2 - p.l1;
        let llen = lvec.len();

        let rvec = p.r2 - p.r1;
        let rlen = rvec.len();

        let form_width = (rlen / FORMATION_SPACING).round() as usize;

        for cur in 0..self.len() {
            /*match self.state[cur] {
                Stationary => continue,
                Marching => {}
            }*/

            let target_offset = Vec2f {
                x: cur.checked_rem(form_width).unwrap_or_default() as f64,
                y: cur.checked_div(form_width).unwrap_or_default() as f64,
            }.rot_align(rvec) * FORMATION_SPACING;

            println!("{:?}", target_offset);

            /*let target_offset = if form_width != 0 {
                Vec2f {
                    x: (cur % form_width) as f64,
                    y: (cur / form_width) as f64,
                } * 50.
            } else {
                Vec2f::default()
            };*/

            let target = p.r1 + target_offset;
            let mut d = target - self.pos[cur];
            let dist = d.len();

            if dist < 1. {
                self.state[cur] = Stationary;
                self.vel[cur] = Vec2f::default();
                continue;
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
            self.vel[cur].clamp( VEL_MAX.min(dist) );

            let heading: f64 = f64::atan2(self.vel[cur].y, self.vel[cur].x);

            if heading.is_normal() {
                self.r[cur] = heading;
            }

            self.pos[cur] += self.vel[cur] * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::Vec2f;

    #[test]
    fn test() {
        let form_width: usize = 0;

        let cur: usize = 1;

        let target_offset = Vec2f {
            x: cur.checked_rem(form_width).unwrap_or_default() as f64,
            y: cur.checked_div(form_width).unwrap_or_default() as f64,
        };

        assert_eq!(target_offset, Vec2f::default());

        let form_width: usize = 5;

        let cur: usize = 14;

        let target_offset = Vec2f {
            x: cur.checked_rem(form_width).unwrap_or_default() as f64,
            y: cur.checked_div(form_width).unwrap_or_default() as f64,
        };
        println!("{:?}", target_offset);


        assert_ne!(target_offset, Vec2f::default());
    }
}

