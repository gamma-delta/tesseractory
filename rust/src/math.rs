//! Glam is just here for the IVec4s and quick dirty math internal to functions;
//! everything else uses wedged

use std::ops::Deref;

use getset::CopyGetters;
use glam::IVec4;

use crate::{type_aliases::*, world::Chunk};

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

  /// Return the chunk this block is in
  pub fn chunk(&self) -> ChunkPos {
    // Int division rounds towards zero but we want to round towards negative
    fn convert1(n: i32) -> i32 {
      if n < 0 {
        (n / Chunk::BREADTH) - 1
      } else {
        n / Chunk::BREADTH
      }
    }
    ChunkPos(IVec4::new(
      convert1(self.x),
      convert1(self.y),
      convert1(self.z),
      convert1(self.w),
    ))
  }
}

/// Coordinate of a chunk
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ChunkPos(pub IVec4);

impl Deref for ChunkPos {
  type Target = IVec4;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ChunkPos {
  pub fn new(x: i32, y: i32, z: i32, w: i32) -> ChunkPos {
    Self(IVec4::new(x, y, z, w))
  }

  pub fn min_block(&self) -> BlockPos {
    fn convert1(n: i32) -> i32 {
      let offset = if n < 0 { 1 } else { 0 };
      n * Chunk::BREADTH + offset
    }
    BlockPos(IVec4::new(
      convert1(self.x),
      convert1(self.y),
      convert1(self.z),
      convert1(self.w),
    ))
  }

  /// If the pos is in this chunk, return its offset
  pub fn contained_offset(&self, pos: BlockPos) -> Option<IVec4> {
    let pos_chunk = pos.chunk();
    if pos_chunk != *self {
      None
    } else {
      Some(pos.0 - self.min_block().0)
    }
  }
}

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Ray {
  origin: Vec4,
  heading: Vec4,
  /// Some abstract measure of "length along the ray."
  start: f32,
  end: f32,
}
impl Ray {
  pub const EPSILON: f32 = 0.0001;
  pub const INFINITY: f32 = 1_000_000.0;

  pub fn new(origin: Vec4, heading: Vec4) -> Self {
    Self {
      origin,
      heading,
      start: Ray::EPSILON,
      end: Ray::INFINITY,
    }
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
