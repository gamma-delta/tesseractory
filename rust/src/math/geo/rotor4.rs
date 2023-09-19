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
      /// Pseudo-scalar quadvec part
      pub p: $f,
    }

    impl $t {
      pub fn new(s: $f, bv: $bv, p: $f) -> Self {
        Self { s, bv, p }
      }

      pub fn identity() -> Self {
        Self::new($f::splat(1.0), $bv::zero(), $f::splat(0.0))
      }

      pub fn from_rotation_between(from: $v, to: $v) -> Self {
        let wedge = $bv::wedge(to, from);
        Self::new(
          $f::splat(1.0) + to.dot(from),
          wedge,
          $f::splat(0.0))
        .normalized()
      }

      #[inline]
      pub fn from_angle_plane(angle: $f, plane: $bv) -> Self {
        let half_angle = angle * $f::splat(0.5);
        let (sin, cos) = half_angle.sin_cos();
        Self::new(cos, plane * -sin, $f::splat(0.0)).normalized()
      }

      pub fn mag_sq(&self) -> $f {
        self.s * self.s + self.bv.mag_sq() + self.p * self.p
      }

      pub fn mag(&self) -> $f {
        self.mag_sq().sqrt()
      }

      #[inline]
      pub fn normalize(&mut self) {
        let mag = self.mag();
        self.s /= mag;
        // my bivec doesn't implement floor lmao
        self.bv *= mag.recip();
        self.p /= mag;
      }

      #[inline]
      #[must_use = "Did you mean to use `.normalize()` to normalize `self` in place?"]
      pub fn normalized(&self) -> Self {
        let mut me = self.clone();
        me.normalize();
        me
      }

      pub fn reverse(&self) -> Self {
        Self::new(self.s, self.bv, self.p)
      }

      #[inline]
      pub fn rot_many(&self, vs: &mut [$v]) {
        let coeffs = self.rot_coefficients();

        for v in vs {
          *v = self.apply_rot_coefficients(coeffs, *v);
        }
      }


      #[inline]
      fn rot_coefficients(&self) -> [$f; 8] {
        [
          self.s * self.s,
          self.bv.xy * self.bv.xy,
          self.bv.xz * self.bv.xz,
          self.bv.xw * self.bv.xw,
          self.bv.yz * self.bv.yz,
          self.bv.yw * self.bv.yw,
          self.bv.zw * self.bv.zw,
          self.p * self.p,
        ]
      }

      #[inline]
      fn apply_rot_coefficients(&self, coeff: [$f; 8], a: $v) -> $v {
        // Write out the values like this to make it easier to
        // copy-paste Joe's code
        let s = self.s;
        let b = self.bv;
        let pxyzw = self.p;
        let [s2, bxy2, bxz2, bxw2, byz2, byw2, bzw2, bxyzw2] = coeff;

        let two = $f::splat(2.0);


        let x = (
              two * a.w * b.xw * s
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
            - two * a.z * b.yw * pxyzw
        );
        let y = (
            - two * a.w * b.xw * b.xy
            - two * a.w * b.xz * pxyzw
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
            + two * a.z * b.yz * s
        );
        let z = (
            - two * a.w * b.xw * b.xz
            + two * a.w * b.xy * pxyzw
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
            + a.z * s2

        );
        let w = (
            - a.w * bxw2
            + a.w * bxy2
            + a.w * bxz2
            - a.w * byw2
            + a.w * byz2
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
            - two * a.z * b.zw * s
        );
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
        self.apply_rot_coefficients(coeff, a)
      }
    }

    impl Mul for $t {
      type Output = Self;
      /// Composition of rotors.
      ///
      /// I did not write this myself thank god
      fn mul(self, rhs: $t) -> Self::Output {
        let a = self;
        let b = rhs;

        Self::new(
          -a.bv.xw*b.bv.xw - a.bv.xy*b.bv.xy - a.bv.xz*b.bv.xz - a.bv.yw*b.bv.yw - a.bv.yz*b.bv.yz - a.bv.zw*b.bv.zw + a.s*b.s,
          $bv::new(
            -a.bv.xw*b.bv.yw + a.bv.xy*b.s - a.bv.xz*b.bv.yz + a.bv.yw*b.bv.xw + a.bv.yz*b.bv.xz + a.s*b.bv.xy,
            -a.bv.xw*b.bv.zw + a.bv.xy*b.bv.yz + a.bv.xz*b.s - a.bv.yz*b.bv.xy + a.bv.zw*b.bv.xw + a.s*b.bv.xz,
            a.bv.xw*b.s + a.bv.xy*b.bv.yw + a.bv.xz*b.bv.zw - a.bv.yw*b.bv.xy - a.bv.zw*b.bv.xz + a.s*b.bv.xw,
            -a.bv.xy*b.bv.xz + a.bv.xz*b.bv.xy - a.bv.yw*b.bv.zw + a.bv.yz*b.s + a.bv.zw*b.bv.yw + a.s*b.bv.yz,
            a.bv.xw*b.bv.xy - a.bv.xy*b.bv.xw + a.bv.yw*b.s + a.bv.yz*b.bv.zw - a.bv.zw*b.bv.yz + a.s*b.bv.yw,
            a.bv.xw*b.bv.xz - a.bv.xz*b.bv.xw + a.bv.yw*b.bv.yz - a.bv.yz*b.bv.yw + a.bv.zw*b.s + a.s*b.bv.zw
         ),
         a.bv.xw*b.bv.yz + a.bv.xy*b.bv.zw - a.bv.xz*b.bv.yw - a.bv.yw*b.bv.xz + a.bv.yz*b.bv.xw + a.bv.zw*b.bv.xy
        )
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
    let p = f32x8::splat(rotor.p);
    Self::new(s, bv, p)
  }
}

#[cfg(test)]
mod tests {
  use crate::math::basis4;

  use super::*;

  #[test]
  fn identities() {
    for axis in 0..=3 {
      let basis = basis4(axis);
      let rot = Rotor4::from_rotation_between(basis, basis);
      assert_eq!(rot, Rotor4::identity());
    }
  }

  #[test]
  fn identities_work() {
    let r = Rotor4::from_angle_plane(0.2, Bivec4::unit_xz());
    let ri = r * Rotor4::identity();
    let ir = Rotor4::identity() * r;
    assert_eq!(r, ri);
    assert_eq!(r, ir);

    let v = Vec4::new(7.0, 6.0, 0.0, 4.0);
    let iv = Rotor4::identity() * v;
    assert_eq!(v, iv);

    let r1 =
      Rotor4::new(0.7604, Bivec4::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6), 0.0)
        .normalized();
    let r2 =
      Rotor4::new(0.9876, Bivec4::new(0.3, 0.1, 0.4, 0.1, 0.5, 0.9), 0.0)
        .normalized();
    let r12 = r2 * r1;
    let ir12 = Rotor4::identity() * r2 * r1;
    assert_eq!(r12, ir12);
  }

  // #[test]
  fn associativity() {
    let r1 =
      Rotor4::new(0.7604, Bivec4::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6), 0.0)
        .normalized();
    let r2 =
      Rotor4::new(0.9876, Bivec4::new(0.3, 0.1, 0.4, 0.1, 0.5, 0.9), 0.0)
        .normalized();
    let r3 =
      Rotor4::new(0.2468, Bivec4::new(0.2, 0.7, 0.2, 0.7, 0.2, 0.7), 0.0)
        .normalized();

    let r123 = r1 * r2 * r3;
    let r1_23 = r1 * (r2 * r3);
    let r12_3 = (r1 * r2) * r3;
    assert_eq!(r123, r12_3);
    assert_eq!(r123, r1_23);
  }
}
