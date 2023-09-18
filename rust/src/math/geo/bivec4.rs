use crate::math::geo::Splat;

use std::ops::{Mul, MulAssign, Neg};
use ultraviolet::{f32x4, f32x8};

macro_rules! bivec4s {
  ($($bn:ident => $t:ident),+) => { $(
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct $bn {
      pub xy: $t,
      pub xz: $t,
      pub xw: $t,
      pub yz: $t,
      pub yw: $t,
      pub zw: $t,
    }

    impl $bn {
      #[inline]
      pub const fn new(
        xy: $t,
        xz: $t,
        xw: $t,
        yz: $t,
        yw: $t,
        zw: $t,
      ) -> Self {
        Self {
          xy,
          xz,
          xw,
          yz,
          yw,
          zw,
        }
      }

      #[inline]
      pub fn zero() -> Self {
        let splat = $t::splat(0.0);
        Self::new(splat, splat, splat, splat, splat, splat)
      }

      #[inline]
      pub fn mag_sq(&self) -> $t {
        (self.xy * self.xy)
          + (self.xz * self.xz)
          + (self.xw * self.xw)
          + (self.yz * self.yz)
        + (self.yw * self.yw)
        + (self.zw * self.zw)
      }

      #[inline]
      pub fn mag(&self) -> $t {
        self.mag_sq().sqrt()
      }
    }

    impl Neg for $bn {
      type Output = Self;

      #[inline]
      fn neg(mut self) -> Self::Output {
        self.xy = -self.xy;
        self.xz = -self.xz;
        self.xw = -self.xw;
        self.yz = -self.yz;
        self.yw = -self.yw;
        self.zw = -self.zw;
        self
      }
    }

    impl Mul<$t> for $bn {
      type Output = Self;

      fn mul(self, rhs: $t) -> Self::Output {
        let mut me = self.clone();
        me *= rhs;
        me
      }
    }

    impl MulAssign<$t> for $bn {
      fn mul_assign(&mut self, rhs: $t) {
        self.xy *= rhs;
        self.xz *= rhs;
        self.xw *= rhs;
        self.yz *= rhs;
        self.yw *= rhs;
        self.zw *= rhs;
      }
    }
  )+ };
}

bivec4s! {
  Bivec4 => f32,
  Bivec4x4 => f32x4,
  Bivec4x8 => f32x8
}

impl Bivec4x8 {
  #[inline]
  pub fn splat(bv: Bivec4) -> Self {
    Self::new(
      f32x8::splat(bv.xy),
      f32x8::splat(bv.xz),
      f32x8::splat(bv.xw),
      f32x8::splat(bv.yz),
      f32x8::splat(bv.yw),
      f32x8::splat(bv.zw),
    )
  }
}
