//use std::ops::Add;
use derive_more::{Add, AddAssign, Display, From, Into, Mul, Neg, Sub, SubAssign};
use graphics::math::Vec2d;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Copy, Clone, PartialEq, Neg, Add, Sub, AddAssign, SubAssign, Mul, Serialize, Deserialize,
)]
pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

pub type Vec2f = Vec2<f64>;

impl Default for Vec2f {
    fn default() -> Self {
        let i: i32 = 0;
        Vec2f { x: 0., y: 0. }
    }
}

impl Into<Vec2d> for Vec2f {
    fn into(self) -> Vec2d {
        [self.x, self.y]
    }
}

impl Vec2f {
    pub fn abs(self) -> Vec2f {
        Vec2f {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn clamp(&mut self, max: f64) {
        let l = self.len();
        if l > max {
            self.x /= l / max;
            self.y /= l / max
        }
    }

    pub fn clampret(self, max: f64) -> Vec2f {
        let l = self.len();
        if l > max {
            return Vec2f {
                x: self.x / l * max,
                y: self.y / l * max,
            };
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::Vec2f;
    use quickcheck::quickcheck;

    fn test() {
        assert_eq!(
            Vec2f { x: 1., y: 0. } + Vec2f { x: 2., y: 3. },
            Vec2f { x: 3., y: 3. }
        )
    }
    quickcheck! {
        fn clamp_works(x1: f64, y1: f64, f: f64) -> bool {
            if !(x1 + y1 +f).is_normal() || f<0.{
                  return true
            }

            let mut v = Vec2f { x: x1, y: y1 };
            v.clamp(f);

            v.len() <= f
        }

        fn prop(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
              if !(x1 + x2 + y1 + y2).is_normal() {
                  return true
              }

              Vec2f { x: x1, y: y1 } + Vec2f { x: x2, y: y2 } == Vec2f { x: x1 + x2, y: y1 + y2 }
        }

        fn mul_by_f64(x1: f64, y1: f64, f: f64) -> bool {
            if !(x1 + y1 +f).is_normal() {
                  return true
            }

            println!("( {} {} )* {}", x1, y1, f);

            Vec2f { x: x1, y: y1 } * f == Vec2f { x: x1 * f, y: y1 * f }
        }

        fn mul_by_i64(x1: f64, y1: f64, i: f64) -> bool {
            if !(x1 + y1 + i).is_normal() {
                  return true
            }

            println!("( {} {} )* {}", x1, y1, i);

            Vec2f { x: x1, y: y1 } * i == Vec2f { x: x1 * i, y: y1 * i }
        }
    }
}
