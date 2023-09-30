//! GPU-friendly representations of stuff

use bytemuck::NoUninit;

use crate::world::foxel::{Foxel, FoxelRepr};

use super::Hexadecitree;

const HIGH_BIT16: u16 = 1 << 15;

#[derive(Debug)]
pub(super) enum BrickPtr {
  Solid(Foxel),
  Pointer(usize),
}

impl BrickPtr {
  pub fn encode(&self) -> BrickPtrRepr {
    BrickPtrRepr(match self {
      &BrickPtr::Solid(f) => f as u16,
      &BrickPtr::Pointer(ptr) => {
        debug_assert!(ptr < Hexadecitree::COMPOSITE_BRICK_COUNT);
        HIGH_BIT16 | (ptr as u16)
      }
    })
  }
}

/// The high bit indicates the type of this thing.
///
/// If it's set, then the remainder is a 15-bit brick index.
/// If not, then the low 8 bits are a foxel.
///
/// This means all 0 is entirely air!
#[derive(Debug, Clone, Copy, NoUninit)]
#[repr(transparent)]
pub(super) struct BrickPtrRepr(u16);

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
pub struct Brick(pub [FoxelRepr; Hexadecitree::FOXELS_PER_BRICK]);

impl Hexadecitree {
  pub fn upload(&self, bytes: &mut Vec<u8>) {
    bytes.clear();
    bytes.extend_from_slice(bytemuck::cast_slice(&*self.grid));
    debug_assert_eq!(bytes.len(), Hexadecitree::BRICKS_BYTES);

    bytes.extend_from_slice(bytemuck::cast_slice(
      self.composite_bricks.as_slice(),
    ));
    if bytes.len() > Hexadecitree::MAX_UPLOAD_BYTE_COUNT {
      panic!("tried to ship {} bytes to the gpu but that was more than the allowed {}", bytes.len(), Hexadecitree::MAX_UPLOAD_BYTE_COUNT);
    }
    // resize it to the size of the image, mandatory
    bytes.resize(Hexadecitree::TRANSFER_IMAGE_SIZE_SQ, 0);
  }
}
