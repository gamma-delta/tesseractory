/*!
A lot of the internal math here uses IVec4s and not BlockPos'es
to indicate that they generally don't refer to the actual position
of a block
*/

pub mod iter;
mod reprs;

#[cfg(test)]
mod tests;

use log::{error, trace};

use crate::{math::BlockPos, Foxel};

use reprs::*;

/// To facilitate passing to the gee poo, some memory shenanigans are in order.
#[derive(Debug)]
pub struct Hexadecitree {
  grid: Box<[BrickPtrRepr; Self::TOTAL_BRICK_COUNT]>,
  composite_bricks: Vec<Brick>,
}

impl Hexadecitree {
  pub const BRICKS_ACROSS_WORLD: usize = 32;
  pub const FOXELS_ACROSS_BRICK: usize = 8;

  /// Doing this means I can store u16 brick pointers.
  ///
  /// This is the total number of non-solid bricks allowed.
  pub const COMPOSITE_BRICK_COUNT: usize = 2usize.pow(12);

  pub const FOXELS_PER_BRICK: usize = Self::FOXELS_ACROSS_BRICK.pow(4);
  pub const TOTAL_BRICK_COUNT: usize = Self::BRICKS_ACROSS_WORLD.pow(4);
  pub const FOXELS_ACROSS_WORLD: usize =
    Self::BRICKS_ACROSS_WORLD * Self::FOXELS_ACROSS_BRICK;

  pub const MIN_COORD: i32 = -(Self::FOXELS_ACROSS_WORLD as i32) / 2;
  pub const MAX_COORD: i32 = (Self::FOXELS_ACROSS_WORLD as i32) / 2 - 1;

  pub const BRICKS_BYTES: usize =
    Self::TOTAL_BRICK_COUNT * std::mem::size_of::<BrickPtrRepr>();
  pub const MAX_COMPOSITE_BRICKS_BYTES: usize =
    Self::COMPOSITE_BRICK_COUNT * std::mem::size_of::<Brick>();

  pub const MAX_UPLOAD_BYTE_COUNT: usize =
    Self::BRICKS_BYTES + Self::MAX_COMPOSITE_BRICKS_BYTES;

  pub const TRANSFER_IMAGE_SIZE: usize = 5000;
  pub const TRANSFER_IMAGE_SIZE_SQ: usize = Self::TRANSFER_IMAGE_SIZE.pow(2);

  pub fn new() -> Self {
    let grid_vec = vec![BrickPtrRepr::entirely_air(); Self::TOTAL_BRICK_COUNT];
    let grid = grid_vec.into_boxed_slice().try_into().unwrap();
    Self {
      grid,
      composite_bricks: Vec::new(),
    }
  }

  pub fn get(&self, pos: BlockPos) -> Option<Foxel> {
    let (grid_idx, brick_idx) = decompose_pos(pos)?;
    match self.grid[grid_idx].decode() {
      BrickPtr::Solid(f) => Some(f),
      BrickPtr::Pointer(ptr) => {
        let Some(bricc) = self.composite_bricks.get(ptr) else {
          error!(
            "when getting, a BrickPtr pointed to {} but only have {} bricks",
            ptr,
            self.composite_bricks.len()
          );
          return None;
        };
        Some(bricc.0[brick_idx].decode())
      }
    }
  }

  /// Return the previous foxel
  pub fn set(
    &mut self,
    pos: BlockPos,
    foxel: Foxel,
  ) -> Result<Foxel, SetFoxelError> {
    let (grid_idx, brick_idx) =
      decompose_pos(pos).ok_or(SetFoxelError::OutOfBounds)?;

    let slot = &mut self.grid[grid_idx];
    match slot.decode() {
      BrickPtr::Pointer(ptr) => {
        let Some(bricc) = self.composite_bricks.get_mut(ptr) else {
          error!(
            "when setting, a BrickPtr pointed to {} but only have {} bricks",
            ptr,
            self.composite_bricks.len()
          );
          // OOB I guess?????
          return Err(SetFoxelError::OutOfBounds);
        };
        error!(
          "setting brick #{} composite #{} idx #{} to {:?}",
          grid_idx, ptr, brick_idx, foxel
        );
        let extant = std::mem::replace(&mut bricc.0[brick_idx], foxel.encode());
        Ok(extant.decode())
      }
      BrickPtr::Solid(fill) => {
        // no change!
        if fill == foxel {
          return Ok(foxel);
        }

        let new_composite_idx = self.composite_bricks.len();
        if new_composite_idx >= Self::COMPOSITE_BRICK_COUNT {
          return Err(SetFoxelError::OutOfMemory);
        }

        // Expand the brick
        let mut brick_vec = vec![fill.encode(); Hexadecitree::FOXELS_PER_BRICK];
        brick_vec[brick_idx] = foxel.encode();
        self
          .composite_bricks
          .push(Brick(brick_vec.try_into().unwrap()));

        let ptr_enc = BrickPtr::Pointer(new_composite_idx);
        *slot = ptr_enc.encode();

        error!(
          "expanding brick #{} of {:?},
          now at composite #{} with inclusion {:?} @ #{}",
          grid_idx, fill, new_composite_idx, foxel, brick_idx
        );

        Ok(fill)
      }
    }
  }

  pub fn composite_brick_count(&self) -> usize {
    self.composite_bricks.len()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetFoxelError {
  OutOfBounds,
  OutOfMemory,
}

fn is_block_in_range(pos: BlockPos) -> bool {
  pos
    .0
    .as_array()
    .into_iter()
    .all(|n| (Hexadecitree::MIN_COORD..=Hexadecitree::MAX_COORD).contains(&n))
}

/// Return the index of the brick it's in, then (if the brick isn't solid)
/// the index of the position in the brick
fn decompose_pos(pos: BlockPos) -> Option<(usize, usize)> {
  if !is_block_in_range(pos) {
    return None;
  }

  let mut grid_idx = 0;
  let mut foxel_idx = 0;
  for v in pos.0.as_array() {
    let foxel_pos =
      v.rem_euclid(Hexadecitree::FOXELS_ACROSS_BRICK as i32) as usize;

    let raw_brick_pos = if v >= 0 {
      v / Hexadecitree::FOXELS_ACROSS_BRICK as i32
    } else {
      v / Hexadecitree::FOXELS_ACROSS_BRICK as i32 - 1
    };
    // Shift so 0,0 is in the center of the bricks
    let brick_pos =
      raw_brick_pos + Hexadecitree::BRICKS_ACROSS_WORLD as i32 / 2;
    debug_assert!(brick_pos >= 0);

    foxel_idx *= Hexadecitree::FOXELS_ACROSS_BRICK;
    foxel_idx |= foxel_pos;
    grid_idx *= Hexadecitree::BRICKS_ACROSS_WORLD;
    grid_idx |= brick_pos as usize;
  }

  Some((grid_idx, foxel_idx))
}
