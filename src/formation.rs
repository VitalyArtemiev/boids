use rand::Rng;
use crate::container::FORMATION_SPACING;
use crate::ops::Vec2f;

pub type FormationFunction = fn(usize, usize) -> Vec2f;

pub fn default_formation(index: usize, _: usize) -> Vec2f {
    let mut rng = rand::thread_rng();
    Vec2f {
        x: rng.gen::<f64>() * FORMATION_SPACING,
        y: rng.gen::<f64>() * FORMATION_SPACING,
    }
}

pub fn crutch() -> FormationFunction {
    default_formation
}

pub fn phalanx_formation(index: usize, width: usize) -> Vec2f {
    Vec2f {
        x: index.checked_rem(width).unwrap_or_default() as f64,
        y: index.checked_div(width).unwrap_or_default() as f64,
    }
}

pub fn p_f(index: usize, width: usize, xdir_norm: Vec2f, ydir_norm: Vec2f) -> Vec2f {
    let x = index.checked_rem(width).unwrap_or_default() as f64;
    let y = index.checked_div(width).unwrap_or_default() as f64;

    xdir_norm * x + ydir_norm * y * FORMATION_SPACING
}
pub fn idle_formation(index: usize, width: usize) -> Vec2f {
    let mut rng = rand::thread_rng();
    let mut x = rng.gen::<f64>();
    let mut y = rng.gen::<f64>();

    while x * x + y * y > 1. {
        x = rng.gen::<f64>();
        y = rng.gen::<f64>();
    }

    Vec2f {
        x: x * FORMATION_SPACING,
        y: y * FORMATION_SPACING,
    }
}
