use godot::prelude::*;

use crate::math::geo::*;

use super::vec4_from_gd;

#[derive(GodotClass, Debug, Clone, Copy)]
#[class()]
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
#[class()]
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
}

fn bv(bivec: Bivec4) -> Gd<GdBivec4> {
  Gd::new(GdBivec4 { inner: bivec })
}

fn r(rotor: Rotor4) -> Gd<GdRotor4> {
  Gd::new(GdRotor4 { inner: rotor })
}
