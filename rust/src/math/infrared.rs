use std::ops::{Mul, MulAssign, Neg};

use ultraviolet::Vec4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bivec4 {
  pub xy: f32,
  pub xz: f32,
  pub xw: f32,
  pub yz: f32,
  pub yw: f32,
  pub zw: f32,
}

impl Bivec4 {
  #[inline]
  pub const fn new(
    xy: f32,
    xz: f32,
    xw: f32,
    yz: f32,
    yw: f32,
    zw: f32,
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
    Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
  }

  #[inline]
  pub fn mag_sq(&self) -> f32 {
    (self.xy * self.xy)
      + (self.xz * self.xz)
      + (self.xw * self.xw)
      + (self.yz * self.yz)
      + (self.yw * self.yw)
      + (self.zw * self.zw)
  }

  #[inline]
  pub fn mag(&self) -> f32 {
    self.mag_sq().sqrt()
  }
}

impl Neg for Bivec4 {
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

impl Mul<f32> for Bivec4 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    let mut me = self.clone();
    me *= rhs;
    me
  }
}

impl MulAssign<f32> for Bivec4 {
  fn mul_assign(&mut self, rhs: f32) {
    self.xy *= rhs;
    self.xz *= rhs;
    self.xw *= rhs;
    self.yz *= rhs;
    self.yw *= rhs;
    self.zw *= rhs;
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotor4 {
  pub s: f32,
  pub bv: Bivec4,
}

impl Rotor4 {
  pub fn new(s: f32, bv: Bivec4) -> Self {
    Self { s, bv }
  }

  pub fn identity() -> Self {
    Self::new(0.0, Bivec4::zero())
  }

  pub fn mag_sq(&self) -> f32 {
    self.s * self.s + self.bv.mag_sq()
  }

  pub fn mag(&self) -> f32 {
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
  fn rot_coefficients(&self) -> [f32; 8] {
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
}

impl Mul<Vec4> for Rotor4 {
  type Output = Vec4;

  /// Rotate the vector.
  ///
  /// https://joesubbi.github.io/code/rotor-code/
  fn mul(self, a: Vec4) -> Self::Output {
    let coeff = self.rot_coefficients();
    apply_rot_coefficients(self.bv, coeff, a)
  }
}

#[inline]
fn apply_rot_coefficients(b: Bivec4, coeff: [f32; 8], a: Vec4) -> Vec4 {
  let [s, s2, bxy2, bxz2, bxw2, byz2, byw2, bzw2] = coeff;
  // easier to copy-paste code like this
  let pxyzw = 1.0;
  let bxyzw = 1.0;
  let bxyzw2 = 1.0;

  let x = (2.0 * a.w * b.xw * s
    + 2.0 * a.w * b.xy * b.yw
    + 2.0 * a.w * b.xz * b.zw
    + 2.0 * a.w * b.yz * pxyzw
    - a.x * bxw2
    - a.x * bxy2
    - a.x * bxz2
    + a.x * byw2
    + a.x * byz2
    + a.x * bzw2
    - a.x * bxyzw2
    + a.x * s2
    - 2.0 * a.y * b.xw * b.yw
    + 2.0 * a.y * b.xy * s
    - 2.0 * a.y * b.xz * b.yz
    + 2.0 * a.y * b.zw * pxyzw
    - 2.0 * a.z * b.xw * b.zw
    + 2.0 * a.z * b.xy * b.yz
    + 2.0 * a.z * b.xz * s
    - 2.0 * a.z * b.yw * pxyzw);
  let y = (-2.0 * a.w * b.xw * b.xy - 2.0 * a.w * b.xz * pxyzw
    + 2.0 * a.w * b.yw * s
    + 2.0 * a.w * b.yz * b.zw
    - 2.0 * a.x * b.xw * b.yw
    - 2.0 * a.x * b.xy * s
    - 2.0 * a.x * b.xz * b.yz
    - 2.0 * a.x * b.zw * pxyzw
    + a.y * bxw2
    - a.y * bxy2
    + a.y * bxz2
    - a.y * byw2
    - a.y * byz2
    + a.y * bzw2
    - a.y * bxyzw2
    + a.y * s2
    + 2.0 * a.z * b.xw * pxyzw
    - 2.0 * a.z * b.xy * b.xz
    - 2.0 * a.z * b.yw * b.zw
    + 2.0 * a.z * b.yz * s);
  let z = (-2.0 * a.w * b.xw * b.xz + 2.0 * a.w * b.xy * pxyzw
    - 2.0 * a.w * b.yw * b.yz
    + 2.0 * a.w * b.zw * s
    - 2.0 * a.x * b.xw * b.zw
    + 2.0 * a.x * b.xy * b.yz
    - 2.0 * a.x * b.xz * s
    + 2.0 * a.x * b.yw * pxyzw
    - 2.0 * a.y * b.xw * pxyzw
    - 2.0 * a.y * b.xy * b.xz
    - 2.0 * a.y * b.yw * b.zw
    - 2.0 * a.y * b.yz * s
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
    - 2.0 * a.x * b.xw * s
    + 2.0 * a.x * b.xy * b.yw
    + 2.0 * a.x * b.xz * b.zw
    - 2.0 * a.x * b.yz * pxyzw
    - 2.0 * a.y * b.xw * b.xy
    + 2.0 * a.y * b.xz * pxyzw
    - 2.0 * a.y * b.yw * s
    + 2.0 * a.y * b.yz * b.zw
    - 2.0 * a.z * b.xw * b.xz
    - 2.0 * a.z * b.xy * pxyzw
    - 2.0 * a.z * b.yw * b.yz
    - 2.0 * a.z * b.zw * s);
  Vec4::new(x, y, z, w)
}
