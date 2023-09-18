pub mod algos;
pub mod geo;
pub mod hexadecitree;

use ultraviolet::{IVec4, Vec4};

use std::ops::Deref;

/// Coordinate of a block
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BlockPos(pub IVec4);

impl Deref for BlockPos {
  type Target = IVec4;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl BlockPos {
  pub fn new(x: i32, y: i32, z: i32, w: i32) -> BlockPos {
    Self(IVec4::new(x, y, z, w))
  }
}

/// To make things less arbitrary, I'm declaring X to be "the axis
/// that gravity goes in," with +X being "up."
///
/// This means that the imaginary direction can be Y, Z, or W.
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Axis {
  X,
  Y,
  Z,
  W,
}

#[inline]
pub fn basis4(idx: usize) -> Vec4 {
  let mut v = Vec4::zero();
  v[idx] = 1.0;
  v
}
