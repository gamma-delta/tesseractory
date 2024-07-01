/*!
A lot of the internal math here uses IVec4s and not BlockPos'es
to indicate that they generally don't refer to the actual position
of a block
*/

pub mod iter;
pub mod reprs;
mod upload;

#[cfg(test)]
mod tests;

use log::{error, trace};
use ultraviolet::IVec4;

use crate::{math::BlockPos, Foxel};

use reprs::*;

/// To facilitate passing to the gee poo, some memory shenanigans are in order.
#[derive(Debug)]
pub struct Hexadecitree {
  brick_ptrs: Box<[BrickPtrRepr; Self::TOTAL_BRICK_COUNT as usize]>,
  composite_bricks: Vec<Brick>,
}

impl Hexadecitree {
  pub const BRICKS_ACROSS_WORLD: u32 = 32;
  pub const FOXELS_ACROSS_BRICK: u32 = 8;

  /// Doing this means I can store u16 brick pointers.
  ///
  /// This is the total number of non-solid bricks allowed.
  pub const COMPOSITE_BRICK_COUNT: u32 = 2u32.pow(12);

  pub const FOXELS_PER_BRICK: u32 = Self::FOXELS_ACROSS_BRICK.pow(4);
  pub const TOTAL_BRICK_COUNT: u32 = Self::BRICKS_ACROSS_WORLD.pow(4);
  pub const FOXELS_ACROSS_WORLD: u32 =
    Self::BRICKS_ACROSS_WORLD * Self::FOXELS_ACROSS_BRICK;

  pub const MIN_COORD: i32 = -(Self::FOXELS_ACROSS_WORLD as i32) / 2;
  pub const MAX_COORD: i32 = (Self::FOXELS_ACROSS_WORLD as i32) / 2 - 1;

  pub fn new() -> Self {
    let grid_vec =
      vec![BrickPtrRepr::entirely_air(); Self::TOTAL_BRICK_COUNT as usize];
    let grid = grid_vec.into_boxed_slice().try_into().unwrap();
    Self {
      brick_ptrs: grid,
      composite_bricks: Vec::new(),
    }
  }

  pub fn get(&self, pos: BlockPos) -> Option<Foxel> {
    let (brick_idx, foxel_idx) = decompose_pos(pos)?;
    let brick_ptr_repr = self.brick_ptrs[brick_idx];
    Some(match self.brick_repr_to_ref(brick_ptr_repr)? {
      BrickRef::Solid(f) => f,
      BrickRef::Ref(bricc) => bricc.0[foxel_idx].decode(),
    })
  }

  /// Return the previous foxel
  pub fn set(
    &mut self,
    pos: BlockPos,
    foxel: Foxel,
  ) -> Result<Foxel, SetFoxelError> {
    let (grid_idx, foxel_idx) =
      decompose_pos(pos).ok_or(SetFoxelError::OutOfBounds)?;

    let slot = &mut self.brick_ptrs[grid_idx];
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
        trace!(
          "setting brick #{} composite #{} idx #{} to {:?}",
          grid_idx,
          ptr,
          foxel_idx,
          foxel
        );
        let extant = std::mem::replace(&mut bricc.0[foxel_idx], foxel.encode());
        Ok(extant.decode())
      }
      BrickPtr::Solid(fill) => {
        // no change!
        if fill == foxel {
          return Ok(foxel);
        }

        let new_composite_idx = self.composite_bricks.len();
        if new_composite_idx >= Self::COMPOSITE_BRICK_COUNT as usize {
          return Err(SetFoxelError::OutOfMemory);
        }

        // Expand the brick
        let mut brick_vec =
          vec![fill.encode(); Hexadecitree::FOXELS_PER_BRICK as usize];
        brick_vec[foxel_idx] = foxel.encode();
        self
          .composite_bricks
          .push(Brick(brick_vec.try_into().unwrap()));

        let ptr_enc = BrickPtr::Pointer(new_composite_idx);
        *slot = ptr_enc.encode();

        trace!(
          "expanding brick #{} of {:?},\
          now at composite #{} with inclusion {:?} @ #{}",
          grid_idx,
          fill,
          new_composite_idx,
          foxel,
          foxel_idx
        );

        Ok(fill)
      }
    }
  }

  pub fn composite_brick_count(&self) -> usize {
    self.composite_bricks.len()
  }

  /// Also yields the smallest block pos in each brick
  pub fn brick_ptrs(&self) -> impl Iterator<Item = (IVec4, BrickPtrRepr)> + '_ {
    self.brick_ptrs.iter().enumerate().map(|(idx, repr)| {
      let iidx = idx as i32;
      let baw = Self::BRICKS_ACROSS_WORLD as i32;
      let raw_brick_pos = IVec4::new(
        iidx % baw,
        iidx / baw % baw,
        iidx / baw / baw % baw,
        iidx / baw / baw / baw,
      );
      // Shift it again
      let brick_pos = raw_brick_pos
        - IVec4::broadcast(Hexadecitree::BRICKS_ACROSS_WORLD as i32 / 2);
      (brick_pos * Self::FOXELS_ACROSS_BRICK as i32, *repr)
    })
  }

  pub fn brick_repr_to_ref(&self, ptr: BrickPtrRepr) -> Option<BrickRef<'_>> {
    match ptr.decode() {
      BrickPtr::Solid(f) => Some(BrickRef::Solid(f)),
      BrickPtr::Pointer(ptr) => {
        let Some(bricc) = self.composite_bricks.get(ptr) else {
          error!(
            "when getting, a BrickPtr pointed to {} but only have {} bricks",
            ptr,
            self.composite_bricks.len()
          );
          return None;
        };
        Some(BrickRef::Ref(bricc))
      }
    }
  }

  pub fn auugh(&self) {
    println!("{:?}", &self.composite_bricks[0]);
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

    grid_idx *= Hexadecitree::BRICKS_ACROSS_WORLD;
    grid_idx |= brick_pos as u32;
    foxel_idx *= Hexadecitree::FOXELS_ACROSS_BRICK;
    foxel_idx |= foxel_pos as u32;
  }

  Some((grid_idx as usize, foxel_idx as usize))
}
