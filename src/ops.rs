//use std::ops::Add;
use graphics::math::Vec2d;
use derive_more::{Add, Sub, Display, From, Into};

#[derive(Debug, Copy, Clone, PartialEq, Add, Sub)]
pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

pub type Vec2f = Vec2<f64>;

impl Default for Vec2f {
    fn default() -> Self {
        let i: i32 = 0;
        Vec2f {x: 0., y: 0.}
    }
}

impl Vec2f {
    fn abs(self) -> Vec2f {
        Vec2f {x: self.x.abs(), y: self.y.abs()}
    }
}

/*impl Add for Vec2d {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}*/

#[cfg(test)]
mod tests {
    use crate::ops::Vec2f;
    use quickcheck::quickcheck;

    fn test() {
        assert_eq!(Vec2f { x: 1., y: 0. } + Vec2f { x: 2., y: 3. }, Vec2f { x: 3., y: 3. })
    }
    quickcheck! {
      fn prop(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
            if !(x1 + x2 + y1 + y2).is_normal() {
                return true
            }

            Vec2f { x: x1, y: y1 } + Vec2f { x: x2, y: y2 } == Vec2f { x: x1 + x2, y: y1 + y2 }
      }
  }
}

