use crate::boids::BoidState::{Marching, Stationary};
use crate::ops::Vec2f;
use crate::player::PlayerState;
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
#[soa_derive = "Serialize, Deserialize"]
pub struct Boid {
    pub pos: Vec2f,
    pub vel: Vec2f,
    pub(crate) r: f64,
    pub state: BoidState,
    pub color: [f32; 4], //todo: so far no reason to store color
}

const SPREAD: f64 = 600.;
const VEL_SPREAD: f64 = 500.;

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
                color: [c, c, c, 1.2 - c],
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
