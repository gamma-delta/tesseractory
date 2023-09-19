use crate::math::geo::Splat;

use std::ops::{Mul, MulAssign, Neg};
use ultraviolet::{f32x4, f32x8, Vec4, Vec4x4, Vec4x8};

macro_rules! bivec4s {
  ($($t:ident $v:ident => $bn:ident),+) => { $(
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
      pub fn unit_xy() -> Self {
        let o = $t::splat(0.0);
        let l = $t::splat(1.0);
        Self::new(l, o, o, o, o, o)
      }
      #[inline]
      pub fn unit_xz() -> Self {
        let o = $t::splat(0.0);
        let l = $t::splat(1.0);
        Self::new(o, l, o, o, o, o)
      }
      #[inline]
      pub fn unit_xw() -> Self {
        let o = $t::splat(0.0);
        let l = $t::splat(1.0);
        Self::new(o, o, l, o, o, o)
      }
      #[inline]
      pub fn unit_yz() -> Self {
        let o = $t::splat(0.0);
        let l = $t::splat(1.0);
        Self::new(o, o, o, l, o, o)
      }
      #[inline]
      pub fn unit_yw() -> Self {
        let o = $t::splat(0.0);
        let l = $t::splat(1.0);
        Self::new(o, o, o, o, l, o)
      }
      #[inline]
      pub fn unit_zw() -> Self {
        let o = $t::splat(0.0);
        let l = $t::splat(1.0);
        Self::new(o, o, o, o, o, l)
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

      /// Return a Bivec4 of the wedge of these two Vec4s
      #[inline]
      pub fn wedge(a: $v, b: $v) -> Self {
        Self::new(
          a.x * b.y - a.y * b.x,
          a.x * b.z - a.z * b.x,
          a.x * b.w - a.w * b.x,
          a.y * b.z - a.z * b.y,
          a.y * b.w - a.w * b.y,
          a.z * b.w - a.w * b.z,
        )
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
  f32 Vec4 => Bivec4,
  f32x4 Vec4x4 => Bivec4x4,
  f32x8 Vec4x8 => Bivec4x8
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
