//! Glam is just here for the IVec4s and quick dirty math internal to functions;
//! everything else uses wedged

use std::ops::Deref;

use glam::IVec4;

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
