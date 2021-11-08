//use std::ops::Add;
use derive_more::{Add, AddAssign, Display, From, Into, Mul, MulAssign, Neg, Sub, SubAssign};
use graphics::math::Vec2d;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Neg,
    Add,
    Sub,
    Mul,
    AddAssign,
    SubAssign,
    MulAssign,
    Serialize,
    Deserialize,
)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
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

    ///Manhattan distance
    pub fn man(self) -> f64 {
        self.x.abs() + self.y.abs()
    }

    pub fn clamp(&mut self, max: f64) {
        let l = self.len();
        if l > max {
            self.x /= l / max;
            self.y /= l / max
        }
    }

    pub fn normalise(&self) -> Vec2f {
        let l = self.len();
        if l > 0. {
            Vec2f {
                x: self.x / l,
                y: self.y / l
            }
        } else {
            *self
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

    pub fn rot_align(self, other: Vec2f) -> Vec2f {
        //find rotation from (1; 0) to (other)

        let den = other.len();

        if den == 0. {
            return self;
        }

        let cos = (other.x) / den; //dot product ~ cos
        let sin = (other.y) / den; //determinant ~ sin

        Vec2f {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::Vec2f;
    use quickcheck::quickcheck;

    #[test]
    fn rot() {
        let v = Vec2f { x: 1., y: 1. }.rot_align(Vec2f { x: 1., y: 1. });
        println!("{:?}", v);

        let v = Vec2f { x: 1., y: 1. }.rot_align(Vec2f { x: -1., y: 1. });
        println!("{:?}", v);

        let v = Vec2f { x: 1., y: 1. }.rot_align(Vec2f { x: -1., y: -1. });
        println!("{:?}", v);

        let v = Vec2f { x: 1., y: 1. }.rot_align(Vec2f { x: 1., y: -1. });
        println!("{:?}", v);
    }

    quickcheck! {
        fn clamp_works(x1: f64, y1: f64, f: f64) -> bool {
            if !(x1 + y1 + f).is_normal() || f < 0.{
                  return true
            }

            let mut v = Vec2f { x: x1, y: y1 };
            v.clamp(f);

            v.len() <= f
        }

        fn rot_correct(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
            if !(x1 + x2 + y1 + y2).is_normal() {
                  return true
              }

            Vec2f { x: x1, y: y1 }.rot_align(Vec2f { x: x2, y: y2 });

            let v = Vec2f {x: 1., y: 0.}.rot_align(Vec2f {x: 1., y: 1.});
            println!("{:?}", v);

            true
        }

        fn add_correct(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
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
