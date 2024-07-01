//! GPU-friendly representations of stuff

use bytemuck::NoUninit;

use crate::world::foxel::{Foxel, FoxelRepr};

use super::Hexadecitree;

const HIGH_BIT16: u16 = 1 << 15;

#[derive(Debug)]
pub enum BrickPtr {
  Solid(Foxel),
  Pointer(usize),
}

impl BrickPtr {
  pub fn encode(&self) -> BrickPtrRepr {
    BrickPtrRepr(match self {
      &BrickPtr::Solid(f) => f as u16,
      &BrickPtr::Pointer(ptr) => {
        debug_assert!(ptr < Hexadecitree::COMPOSITE_BRICK_COUNT as usize);
        HIGH_BIT16 | (ptr as u16)
      }
    })
  }
}

/// Friendly helper for an actual Rust reference to the brick
pub enum BrickRef<'a> {
  Solid(Foxel),
  Ref(&'a Brick),
}

/// The high bit indicates the type of this thing.
///
/// If it's set, then the remainder is a 15-bit brick index.
/// If not, then the low 8 bits are a foxel.
///
/// This means all 0 is entirely air!
#[derive(Debug, Clone, Copy, NoUninit)]
#[repr(transparent)]
pub struct BrickPtrRepr(pub u16);

impl BrickPtrRepr {
  pub fn entirely_air() -> Self {
    Self(0)
  }

  pub fn decode(self) -> BrickPtr {
    let x = self.0;

    if x & HIGH_BIT16 != 0 {
      let ptr = x & (!HIGH_BIT16);
      BrickPtr::Pointer(ptr as usize)
    } else {
      let foxel = u8::try_from(x & 0xff)
        .ok()
        .and_then(|i| Foxel::try_from(i).ok())
        .unwrap_or(Foxel::Invalid);
      BrickPtr::Solid(foxel)
    }
  }
}

#[derive(Debug, Clone, Copy, NoUninit)]
#[repr(transparent)]
pub struct Brick(pub [FoxelRepr; Hexadecitree::FOXELS_PER_BRICK as usize]);
