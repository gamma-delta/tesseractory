use godot::prelude::*;

use crate::math::geo::*;

use super::{vec4_from_gd, vec4_to_gd};

#[derive(GodotClass, Debug, Clone, Copy)]
#[class(no_init)]
#[allow(dead_code)]
pub struct GdBivec4 {
  pub inner: Bivec4,
}

#[godot_api]
#[allow(dead_code)]
impl GdBivec4 {
  // because this is itself a macro i can't use macros
  // to help with codegen. auwuhg.
  #[func]
  fn zero() -> Gd<GdBivec4> {
    bv(Bivec4::zero())
  }

  #[func]
  fn unit_xy() -> Gd<GdBivec4> {
    bv(Bivec4::unit_xy())
  }
  #[func]
  fn unit_xz() -> Gd<GdBivec4> {
    bv(Bivec4::unit_xz())
  }
  #[func]
  fn unit_xw() -> Gd<GdBivec4> {
    bv(Bivec4::unit_xw())
  }
  #[func]
  fn unit_yz() -> Gd<GdBivec4> {
    bv(Bivec4::unit_yz())
  }
  #[func]
  fn unit_yw() -> Gd<GdBivec4> {
    bv(Bivec4::unit_yw())
  }
  #[func]
  fn unit_zw() -> Gd<GdBivec4> {
    bv(Bivec4::unit_zw())
  }
}

#[derive(GodotClass, Debug, Clone, Copy)]
#[class(no_init)]
#[allow(dead_code)]
pub struct GdRotor4 {
  pub inner: Rotor4,
}

#[godot_api]
#[allow(dead_code)]
impl GdRotor4 {
  #[func]
  fn identity() -> Gd<GdRotor4> {
    r(Rotor4::identity())
  }

  #[func]
  fn from_rotation_between(from: Vector4, to: Vector4) -> Gd<GdRotor4> {
    r(Rotor4::from_rotation_between(
      vec4_from_gd(from),
      vec4_from_gd(to),
    ))
  }

  #[func]
  fn from_angle_plane(angle: f32, plane: Gd<GdBivec4>) -> Gd<GdRotor4> {
    r(Rotor4::from_angle_plane(angle, plane.bind().inner))
  }

  #[func]
  fn composed(&self, rhs: Gd<GdRotor4>) -> Gd<GdRotor4> {
    r(self.inner * rhs.bind().inner)
  }

  #[func]
  fn transform_vec(&self, rhs: Vector4) -> Vector4 {
    vec4_to_gd(self.inner * vec4_from_gd(rhs))
  }

  #[func]
  fn splat_to_array(&self) -> PackedFloat32Array {
    PackedFloat32Array::from(&[
      self.inner.s,
      self.inner.bv.xy,
      self.inner.bv.xz,
      self.inner.bv.xw,
      self.inner.bv.yz,
      self.inner.bv.yw,
      self.inner.bv.zw,
      self.inner.p,
    ])
  }
}

fn bv(bivec: Bivec4) -> Gd<GdBivec4> {
  Gd::from_object(GdBivec4 { inner: bivec })
}

fn r(rotor: Rotor4) -> Gd<GdRotor4> {
  Gd::from_object(GdRotor4 { inner: rotor })
}
