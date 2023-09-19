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
#[repr(transparent)]
pub struct Hexadecitree {
  /// 0th idx is always the root
  arena: Vec<TreeLevel>,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
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
#[repr(C, u8)]
enum TreeLevel {
  Empty,
  /// Indices into the branch arena.
  /// Index this by going to the tree ref, then adding the child pointer.
  Branch(TreeRef),
  Leaf([Foxel; 16]),
}

impl Hexadecitree {
  /// The 16th level down (idx 15) is the trees.
  pub const DEPTH: usize = 16;
  pub const MAX_COORD: i32 = 2i32.pow(Self::DEPTH as u32);
  pub const MIN_COORD: i32 = -2i32.pow(Self::DEPTH as u32) - 1;

  pub const CHILDREN: usize = 16; // coincidence

  pub fn new() -> Self {
    let root = TreeLevel::Branch(TreeRef(1));
    let mut arena = vec![root];
    for _ in 0..Self::CHILDREN {
      arena.push(TreeLevel::Empty);
    }
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

/// Returns an optional ptr to the lowest level.
///
/// The arena[ptr] will always be a Leaf. Index into it with the returned idx.
fn find_recurse(
  arena: &[TreeLevel],
  tree_ref: TreeRef,
  pos: IVec4,
  depth: usize,
) -> Option<(TreeRef, u8)> {
  let indent = " ".repeat(depth);

  let (child_idx, pos2) = step_down_pos(pos, depth);
  let child_idx = child_idx as usize;

  let tree = arena.get(tree_ref.0 as usize).unwrap();
  println!(
    "{}: scanning {:?} -> {:?}, idx {}",
    indent, tree_ref, &tree, child_idx
  );

  if depth == Hexadecitree::DEPTH - 1 {
    // yes! better find a leaf node here
    let TreeLevel::Leaf(ref _foxels) = tree else {
      panic!("tried to get foxels out of a branch node")
    };
    println!("{}: found foxels at {:?}", indent, tree_ref);
    Some((tree_ref, child_idx as u8))
  } else {
    // Indexing down
    let branch_idx = match tree {
      TreeLevel::Branch(b) => b,
      TreeLevel::Empty => return None,
      TreeLevel::Leaf(_) => panic!("tried to get branches out of a leaf node"),
    };
    let next_level_ptr = TreeRef(branch_idx.0 + child_idx as u32);
    println!("{}: going to {:?}", indent, next_level_ptr);
    find_recurse(arena, next_level_ptr, pos2, depth + 1)
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
  let indent = " ".repeat(depth);

  let (child_idx, pos2) = step_down_pos(pos, depth);
  let child_idx = child_idx as usize;

  let old_len = arena.len();
  let tree = arena.get_mut(tree_ref.0 as usize).unwrap();

  if depth == Hexadecitree::DEPTH - 1 {
    // Bottom of tree
    let foxels = match tree {
      TreeLevel::Empty => panic!("tried to access an empty branch"),
      TreeLevel::Leaf(f) => f,
      TreeLevel::Branch(_) => {
        panic!("tried to get foxels out of a branch")
      }
    };

    let extant = std::mem::replace(&mut foxels[child_idx], foxel);
    println!(
      "{}: found foxel slot {:?}, setting {:?}",
      indent, extant, foxel
    );
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
          TreeLevel::Leaf([Foxel::Air; 16])
        } else {
          // This has no children, yet
          TreeLevel::Empty
        };
        // yes! we can now use the level ptr
        let new_level_ptr = TreeRef(old_len as u32 + child_idx as u32);
        *tree = TreeLevel::Branch(new_level_ptr);
        // Create the space for the 16 children
        arena.push(new_level);
        arena.extend(
          std::iter::from_fn(|| Some(TreeLevel::Empty))
            .take(Hexadecitree::CHILDREN),
        );
        println!(
          "{}: setting {:?} to point to {:?}; {} + {}",
          indent, tree_ref, new_level_ptr, old_len, child_idx
        );

        set_foxel_recurse(arena, new_level_ptr, pos2, foxel, depth + 1, true)
      }
      &mut TreeLevel::Branch(subtree_ptr) => {
        let next_idx = TreeRef(subtree_ptr.0 + child_idx as u32);
        println!(
          "{}: found old branch at {:?}; off to {:?} + {} = {:?}",
          indent, tree_ref, subtree_ptr, child_idx, next_idx,
        );

        set_foxel_recurse(arena, next_idx, pos2, foxel, depth + 1, ever_failed)
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
