/*!
A lot of the internal math here uses IVec4s and not BlockPos'es
to indicate that they generally don't refer to the actual position
of a block
*/

pub mod iter;
mod reprs;

#[cfg(test)]
mod tests;

use ultraviolet::IVec4;

use crate::{math::BlockPos, Foxel};

use reprs::*;

/// To facilitate passing to the gee poo, some memory shenanigans are in order.
#[derive(Debug)]
pub struct Hexadecitree {
  /// 0th idx is always the root
  arena: Vec<TreeLevelInner>,
  foxel_arena: Vec<Foxel>,
}

impl Hexadecitree {
  pub const DEPTH: usize = 6;
  pub const MAX_COORD: i32 =
    (Self::BRANCH_HYPERSIZE as i32).pow(Self::DEPTH as u32 - 1);
  pub const MIN_COORD: i32 =
    -(Self::BRANCH_HYPERSIZE as i32).pow(Self::DEPTH as u32 - 1) - 1;

  pub const BRANCH_HYPERSIZE: usize = 4;
  pub const CHILDREN: usize = Self::BRANCH_HYPERSIZE.pow(4);

  pub fn new() -> Self {
    let root = TreeLevel::Branch(TreeRef::from_idx(1)).enc();
    let mut arena = vec![root];
    for _ in 0..Self::CHILDREN {
      arena.push(TreeLevel::Empty.enc());
    }
    let foxel_arena = Vec::new();
    Self { arena, foxel_arena }
  }

  pub fn get(&self, pos: BlockPos) -> Option<Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      let idx = self.find_recurse(TreeRef::root(), pos.0, 0)?;
      Some(self.foxel_arena[idx])
    }
  }

  pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      let idx = self.find_recurse(TreeRef::root(), pos.0, 0)?;
      Some(&mut self.foxel_arena[idx])
    }
  }

  /// Return the previous foxel
  pub fn set(&mut self, pos: BlockPos, foxel: Foxel) -> Option<Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      self.set_foxel_recurse(TreeRef::root(), pos.0, foxel, 0, false)
    }
  }

  /// `(len, cap)`
  pub fn branch_sizes(&self) -> (usize, usize) {
    (self.arena.len(), self.arena.capacity())
  }

  /// `(len, cap)`
  pub fn foxel_sizes(&self) -> (usize, usize) {
    (self.foxel_arena.len(), self.foxel_arena.capacity())
  }

  pub fn memory(&self) -> usize {
    self.arena.capacity() * std::mem::size_of::<TreeLevelInner>()
      + self.foxel_arena.capacity() * std::mem::size_of::<Foxel>()
  }

  /// Returns an optional ptr to the lowest level.
  ///
  /// The arena[ptr] will always be a Leaf. Index into it with the returned idx.
  fn find_recurse(
    &self,
    tree_ref: TreeRef,
    pos: IVec4,
    depth: usize,
  ) -> Option<usize> {
    let (child_idx, pos2) = step_down_pos(pos, depth);

    let tree = self.arena.get(tree_ref.idx()).unwrap().friendly();

    if depth == Hexadecitree::DEPTH - 1 {
      // yes! better find a leaf node here
      let TreeLevel::Leaf(foxels) = tree else {
        panic!("tried to get foxels out of a branch node")
      };
      Some(foxels.idx(child_idx))
    } else {
      // Indexing down
      let branch_idx = match tree {
        TreeLevel::Branch(b) => b,
        TreeLevel::Empty => return None,
        TreeLevel::Leaf(_) => {
          panic!("tried to get branches out of a leaf node")
        }
      };
      let next_level_ptr = TreeRef::from_idx(branch_idx.child_idx(child_idx));
      self.find_recurse(next_level_ptr, pos2, depth + 1)
    }
  }

  /// Call with `None` foxel to just create all the needed branches
  fn set_foxel_recurse(
    &mut self,
    tree_ref: TreeRef,
    pos: IVec4,
    foxel: Foxel,
    depth: usize,
    ever_failed: bool,
  ) -> Option<Foxel> {
    let (child_idx, pos2) = step_down_pos(pos, depth);

    let old_len = self.arena.len();
    let tree_slot = self.arena.get_mut(tree_ref.idx()).unwrap();
    let tree = tree_slot.friendly();

    if depth == Hexadecitree::DEPTH - 1 {
      // Bottom of tree
      let foxels = match tree {
        TreeLevel::Empty => panic!("tried to access an empty branch"),
        TreeLevel::Leaf(f) => f,
        TreeLevel::Branch(_) => {
          panic!("tried to get foxels out of a branch")
        }
      };

      let extant =
        std::mem::replace(&mut self.foxel_arena[foxels.idx(child_idx)], foxel);
      if ever_failed {
        // it'll be air
        None
      } else {
        Some(extant)
      }
    } else {
      match tree {
        TreeLevel::Leaf(_) => panic!("tried to get branches out of a leaf"),
        TreeLevel::Empty => {
          // Create a new branch
          let new_level = if depth == Hexadecitree::DEPTH - 2 {
            let len = self.foxel_arena.len();
            self
              .foxel_arena
              .extend_from_slice(&[Foxel::Air; Hexadecitree::CHILDREN]);
            TreeLevel::Leaf(FoxelSpanRef::from_idx(len as _))
          } else {
            // This has no children, yet
            TreeLevel::Empty
          };
          // yes! we can now use the level ptr
          let span_ptr = TreeRef::from_idx(old_len);
          *tree_slot = TreeLevel::Branch(span_ptr).enc();
          // Create the space for the 16 children
          self.arena.push(new_level.enc());
          self.arena.extend(
            std::iter::from_fn(|| Some(TreeLevel::Empty.enc()))
              .take(Hexadecitree::CHILDREN),
          );
          let new_level_ptr = TreeRef::from_idx(span_ptr.child_idx(child_idx));

          self.set_foxel_recurse(new_level_ptr, pos2, foxel, depth + 1, true)
        }
        TreeLevel::Branch(subtree_ptr) => {
          let next_idx = TreeRef::from_idx(subtree_ptr.child_idx(child_idx));

          self.set_foxel_recurse(next_idx, pos2, foxel, depth + 1, ever_failed)
        }
      }
    }
  }
}

fn is_block_in_range(pos: BlockPos) -> bool {
  pos
    .0
    .as_array()
    .into_iter()
    .all(|n| (Hexadecitree::MIN_COORD..=Hexadecitree::MAX_COORD).contains(&n))
}

/// Step down twice.
///
/// Indexing uses all of the byte now. `WXYZWXYZ`, where the lower
/// happens first.
fn step_down_pos(pos: IVec4, depth: usize) -> (u8, IVec4) {
  let (idx1, pos1) = step_down_one(pos, depth);
  let (idx2, pos2) = step_down_one(pos1, depth);
  (idx2 << 4 | idx1, pos2)
}

/// Return the index in the children, and the next "block pos"
/// to examine.
fn step_down_one(pos: IVec4, depth: usize) -> (u8, IVec4) {
  debug_assert!(
    depth < Hexadecitree::DEPTH,
    "tried to iterate too many layers down"
  );

  if depth == 0 {
    let positive = ((pos.x >= 0) as u8)
      | ((pos.y >= 0) as u8) << 1
      | ((pos.z >= 0) as u8) << 2
      | ((pos.w >= 0) as u8) << 3;
    (positive, pos.abs())
  } else {
    let one_bits = ((pos.x & 1 != 0) as u8)
      | ((pos.y & 1 != 0) as u8) << 1
      | ((pos.z & 1 != 0) as u8) << 2
      | ((pos.w & 1 != 0) as u8) << 3;
    (one_bits, pos / 2) // shl 1
  }
}
