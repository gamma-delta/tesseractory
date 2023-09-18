use crate::math::geo::{Bivec4, Bivec4x4, Bivec4x8, Splat};

use std::ops::Mul;

use ultraviolet::{f32x4, f32x8, Vec4, Vec4x4, Vec4x8};

macro_rules! rotor4s {
  ($($f:ident $bv:ident $v:ident => $t:ident),+) => { $(
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct $t {
      pub s: $f,
      pub bv: $bv,
    }

    impl $t {
      pub fn new(s: $f, bv: $bv) -> Self {
        Self { s, bv }
      }

      pub fn identity() -> Self {
        Self::new($f::splat(1.0), $bv::zero())
      }

      pub fn mag_sq(&self) -> $f {
        self.s * self.s + self.bv.mag_sq()
      }

      pub fn mag(&self) -> $f {
        self.mag_sq().sqrt()
      }

      #[inline]
      pub fn normalize(&mut self) {
        let mag = self.mag();
        self.s /= mag;
        self.bv *= mag.recip();
      }

      #[inline]
      #[must_use = "Did you mean to use `.normalize()` to normalize `self` in place?"]
      pub fn normalized(&self) -> Self {
        let mut me = self.clone();
        me.normalize();
        me
      }

      #[inline]
      pub fn rot_many(&self, vs: &mut [$v]) {
        let coeffs = self.rot_coefficients();

        for v in vs {
          *v = Self::apply_rot_coefficients(self.bv, coeffs, *v);
        }
      }

      #[inline]
      fn rot_coefficients(&self) -> [$f; 8] {
        [
          self.s,
          self.s * self.s,
          self.bv.xy * self.bv.xy,
          self.bv.xz * self.bv.xz,
          self.bv.xw * self.bv.xw,
          self.bv.yz * self.bv.yz,
          self.bv.yw * self.bv.yw,
          self.bv.zw * self.bv.zw,
        ]
      }

      #[inline]
      fn apply_rot_coefficients(b: $bv, coeff: [$f; 8], a: $v) -> $v {
        let [s, s2, bxy2, bxz2, bxw2, byz2, byw2, bzw2] = coeff;
        // easier to copy-paste code like this
        let pxyzw = $f::splat(0.0);
        let bxyzw2 = $f::splat(0.0);
        let two = $f::splat(2.0);

        let x = (two * a.w * b.xw * s
          + two * a.w * b.xy * b.yw
          + two * a.w * b.xz * b.zw
          + two * a.w * b.yz * pxyzw
          - a.x * bxw2
          - a.x * bxy2
          - a.x * bxz2
          + a.x * byw2
          + a.x * byz2
          + a.x * bzw2
          - a.x * bxyzw2
          + a.x * s2
          - two * a.y * b.xw * b.yw
          + two * a.y * b.xy * s
          - two * a.y * b.xz * b.yz
          + two * a.y * b.zw * pxyzw
          - two * a.z * b.xw * b.zw
          + two * a.z * b.xy * b.yz
          + two * a.z * b.xz * s
          - two * a.z * b.yw * pxyzw);
        let y = (-two * a.w * b.xw * b.xy - two * a.w * b.xz * pxyzw
          + two * a.w * b.yw * s
          + two * a.w * b.yz * b.zw
          - two * a.x * b.xw * b.yw
          - two * a.x * b.xy * s
          - two * a.x * b.xz * b.yz
          - two * a.x * b.zw * pxyzw
          + a.y * bxw2
          - a.y * bxy2
          + a.y * bxz2
          - a.y * byw2
          - a.y * byz2
          + a.y * bzw2
          - a.y * bxyzw2
          + a.y * s2
          + two * a.z * b.xw * pxyzw
          - two * a.z * b.xy * b.xz
          - two * a.z * b.yw * b.zw
          + two * a.z * b.yz * s);
        let z = (-two * a.w * b.xw * b.xz + two * a.w * b.xy * pxyzw
          - two * a.w * b.yw * b.yz
          + two * a.w * b.zw * s
          - two * a.x * b.xw * b.zw
          + two * a.x * b.xy * b.yz
          - two * a.x * b.xz * s
          + two * a.x * b.yw * pxyzw
          - two * a.y * b.xw * pxyzw
          - two * a.y * b.xy * b.xz
          - two * a.y * b.yw * b.zw
          - two * a.y * b.yz * s
          + a.z * bxw2
          + a.z * bxy2
          - a.z * bxz2
          + a.z * byw2
          - a.z * byz2
          - a.z * bzw2
          - a.z * bxyzw2
          + a.z * s2);
        let w = (-a.w * bxw2 + a.w * bxy2 + a.w * bxz2 - a.w * byw2 + a.w * byz2
          - a.w * bzw2
          - a.w * bxyzw2
          + a.w * s2
          - two * a.x * b.xw * s
          + two * a.x * b.xy * b.yw
          + two * a.x * b.xz * b.zw
          - two * a.x * b.yz * pxyzw
          - two * a.y * b.xw * b.xy
          + two * a.y * b.xz * pxyzw
          - two * a.y * b.yw * s
          + two * a.y * b.yz * b.zw
          - two * a.z * b.xw * b.xz
          - two * a.z * b.xy * pxyzw
          - two * a.z * b.yw * b.yz
          - two * a.z * b.zw * s);
        $v::new(x, y, z, w)
      }
    }

    impl Mul<$v> for $t {
      type Output = $v;

      /// Rotate the vector.
      ///
      /// https://joesubbi.github.io/code/rotor-code/
      fn mul(self, a: $v) -> Self::Output {
        let coeff = self.rot_coefficients();
        Self::apply_rot_coefficients(self.bv, coeff, a)
      }
    }
  )+ };
}

rotor4s! {
  f32 Bivec4 Vec4 => Rotor4,
  f32x4 Bivec4x4 Vec4x4 => Rotor4x4,
  f32x8 Bivec4x8 Vec4x8 => Rotor4x8
}

impl Rotor4x8 {
  pub fn splat(rotor: Rotor4) -> Self {
    let s = f32x8::splat(rotor.s);
    let bv = Bivec4x8::splat(rotor.bv);
    Self::new(s, bv)
  }
}
