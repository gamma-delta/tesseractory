/*!
A lot of the internal math here uses IVec4s and not BlockPos'es
to indicate that they generally don't refer to the actual position
of a block

Indexes! For layer 0, it's 0 for negative and 1 for positive.
All other layers just use bitmasking. `WZYX` where X is LSB

*/

pub mod iter;

#[cfg(test)]
mod tests;

use ultraviolet::IVec4;

use crate::math::BlockPos;

use crate::Foxel;

/// To facilitate passing to the gee poo, some memory shenanigans are in order.
#[derive(Debug)]
pub struct Hexadecitree {
  /// 0th idx is always the root
  arena: Vec<TreeLevel>,
}

impl Hexadecitree {
  /// The 16th level down (idx 15) is the trees.
  pub const DEPTH: usize = 16;
  pub const MAX_COORD: i32 = 2i32.pow(Self::DEPTH as u32);
  pub const MIN_COORD: i32 = -2i32.pow(Self::DEPTH as u32) - 1;

  pub fn new() -> Self {
    let root = TreeLevel::Branch(empty_branch());
    let arena = vec![root];
    Self { arena }
  }

  pub fn get(&self, pos: BlockPos) -> Option<Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      let (tree_ref, kid_idx) =
        find_recurse(&self.arena, TreeRef::root(), pos.0, 0)?;
      let tree = &self.arena[tree_ref.0 as usize];
      let TreeLevel::Leaf(ref foxels) = tree else {
        unreachable!()
      };
      Some(foxels[kid_idx as usize])
    }
  }

  pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      let (tree_ref, kid_idx) =
        find_recurse(&self.arena, TreeRef::root(), pos.0, 0)?;
      let tree = &mut self.arena[tree_ref.0 as usize];
      let TreeLevel::Leaf(ref mut foxels) = tree else {
        unreachable!()
      };
      Some(&mut foxels[kid_idx as usize])
    }
  }

  /// Return the previous foxel
  pub fn set(&mut self, pos: BlockPos, foxel: Foxel) -> Option<Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      set_foxel_recurse(
        &mut self.arena,
        TreeRef::root(),
        pos.0,
        foxel,
        0,
        false,
      )
    }
  }
}

/// Call with `None` foxel to just create all the needed branches
fn set_foxel_recurse(
  arena: &mut Vec<TreeLevel>,
  tree_ref: TreeRef,
  pos: IVec4,
  foxel: Foxel,
  depth: usize,
  ever_failed: bool,
) -> Option<Foxel> {
  let (child_idx, pos2) = step_down_pos(pos, depth);
  let child_idx = child_idx as usize;

  let possible_level_ptr = arena.len();
  let tree = arena.get_mut(tree_ref.0 as usize).unwrap();

  if depth == Hexadecitree::DEPTH - 1 {
    // Bottom of tree
    let foxels = match tree {
      TreeLevel::Leaf(f) => f,
      TreeLevel::Branch(_) => {
        panic!("tried to get foxels out of a branch")
      }
    };

    let extant = std::mem::replace(&mut foxels[child_idx], foxel);
    if ever_failed {
      None
    } else {
      Some(extant)
    }
  } else {
    let branches = match tree {
      TreeLevel::Branch(b) => b,
      TreeLevel::Leaf(_) => panic!("tried to get branches out of a leaf"),
    };

    match branches[child_idx] {
      None => {
        // Create a new branch
        let new_level = if depth == Hexadecitree::DEPTH - 2 {
          TreeLevel::Leaf([Foxel::Air; 16])
        } else {
          TreeLevel::Branch(empty_branch())
        };
        // yes! we can now use the level ptr
        let level_ptr = TreeRef(possible_level_ptr as _);
        branches[child_idx] = Some(level_ptr);
        arena.push(new_level);
        set_foxel_recurse(arena, level_ptr, pos2, foxel, depth + 1, true)
      }
      Some(subtree_ptr) => set_foxel_recurse(
        arena,
        subtree_ptr as _,
        pos2,
        foxel,
        depth + 1,
        ever_failed,
      ),
    }
  }
}

/// Returns an optional ptr to the lowest level.
///
/// The arena[ptr] will always be a Leaf. Index into it with the returned idx.
fn find_recurse(
  arena: &[TreeLevel],
  tree_ref: TreeRef,
  pos: IVec4,
  depth: usize,
) -> Option<(TreeRef, u8)> {
  let (child_idx, pos2) = step_down_pos(pos, depth);
  let child_idx = child_idx as usize;

  let tree = arena.get(tree_ref.0 as usize).unwrap();

  if depth == Hexadecitree::DEPTH - 1 {
    // yes! better find a leaf node here
    if !matches!(tree, TreeLevel::Leaf(..)) {
      panic!("tried to get foxels out of a branch node")
    } else {
      Some((tree_ref, child_idx as u8))
    }
  } else {
    // Indexing down
    let TreeLevel::Branch(ref b) = tree else {
      panic!("tried to get branches out of a leaf node")
    };
    let next_level = b[child_idx]?;
    find_recurse(arena, next_level, pos2, depth + 1)
  }
}

#[derive(Clone, Copy)]
struct TreeRef(u32);

impl TreeRef {
  fn root() -> Self {
    Self(0)
  }
}

impl std::fmt::Debug for TreeRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
enum TreeLevel {
  /// Indices into the branch arena
  Branch([Option<TreeRef>; 16]),
  Leaf([Foxel; 16]),
}

fn is_block_in_range(pos: BlockPos) -> bool {
  pos
    .0
    .as_array()
    .into_iter()
    .all(|n| (Hexadecitree::MIN_COORD..=Hexadecitree::MAX_COORD).contains(&n))
}

/// Return the index in the children, and the next "block pos"
/// to examine.
fn step_down_pos(pos: IVec4, depth: usize) -> (u8, IVec4) {
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

fn empty_branch() -> [Option<TreeRef>; 16] {
  // ghughjkhfhg. but snazzy! makes None
  Default::default()
}
