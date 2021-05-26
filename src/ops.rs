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
        return Vec2f {x: self.x.abs(), y: self.y.abs()}
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
    fn test() {
        assert_eq!(Vec2f { x: 1, y: 0 } + Vec2f { x: 2, y: 3 }, Vec2f { x: 3, y: 3 })
    }
    quickcheck! {
      fn prop(xs: Vec<u32>) -> bool {
          xs == reverse(&reverse(&xs))
      }
  }
}

