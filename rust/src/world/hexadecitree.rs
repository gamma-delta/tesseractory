/*!
A lot of the internal math here uses IVec4s and not BlockPos'es
to indicate that they generally don't refer to the actual position
of a block

Indexes! For layer 0, it's 0 for negative and 1 for positive.
All other layers just use bitmasking. `WZYX` where X is LSB

*/

use glam::IVec4;

use crate::math::BlockPos;

use super::Foxel;

pub struct Hexadecitree {
  root: TreeLevel,
}

impl Hexadecitree {
  /// The 16th level down (idx 15) is the trees.
  pub const DEPTH: usize = 16;
  pub const MAX_COORD: i32 = 2i32.pow(Self::DEPTH as u32);
  pub const MIN_COORD: i32 = -2i32.pow(Self::DEPTH as u32) - 1;

  pub fn new() -> Self {
    Self {
      root: TreeLevel::Branch(empty_branch()),
    }
  }

  pub fn get(&self, pos: BlockPos) -> Option<Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      let foxel = IdxRef::recurse_down(&self.root, pos.0, 0);
      foxel.copied()
    }
  }

  pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      IdxMut::recurse_down(&mut self.root, pos.0, 0)
    }
  }

  /// Return the previous foxel
  pub fn set(&mut self, pos: BlockPos, foxel: Foxel) -> Option<Foxel> {
    if !is_block_in_range(pos) {
      None
    } else {
      set_foxel_recurse(&mut self.root, pos.0, foxel, 0, false)
    }
  }
}

/// Call with `None` foxel to just create all the needed branches
fn set_foxel_recurse(
  tree: &mut TreeLevel,
  pos: IVec4,
  foxel: Foxel,
  depth: usize,
  ever_failed: bool,
) -> Option<Foxel> {
  let (idx, pos2) = step_down_pos(pos, depth);
  let idx = idx as usize;

  if depth == Hexadecitree::DEPTH - 1 {
    // Bottom of tree
    let foxels = match tree {
      TreeLevel::Leaf(f) => f,
      TreeLevel::Branch(_) => {
        panic!("tried to get foxels out of a branch")
      }
    };

    let extant = std::mem::replace(&mut foxels[idx], foxel);
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

    match branches[idx] {
      Some(ref mut subtree) => {
        set_foxel_recurse(subtree, pos2, foxel, depth + 1, ever_failed)
      }
      None => {
        // Create a new branch
        let new_level = if depth == Hexadecitree::DEPTH - 2 {
          TreeLevel::Leaf(Box::new([Foxel::Air; 16]))
        } else {
          TreeLevel::Branch(empty_branch())
        };
        let new_level_reborrow = branches[idx].insert(new_level);
        set_foxel_recurse(new_level_reborrow, pos2, foxel, depth + 1, true)
      }
    }
  }
}

enum TreeLevel {
  Branch(Box<[Option<TreeLevel>; 16]>),
  Leaf(Box<[Foxel; 16]>),
}

fn is_block_in_range(pos: BlockPos) -> bool {
  pos.0.cmple(IVec4::splat(Hexadecitree::MAX_COORD)).all()
    && pos.cmpge(IVec4::splat(Hexadecitree::MIN_COORD)).all()
}

/// Return the index in the children, and the next "block pos"
/// to examine.
fn step_down_pos(pos: IVec4, depth: usize) -> (u8, IVec4) {
  debug_assert!(
    depth < Hexadecitree::DEPTH,
    "tried to iterate too many layers down"
  );

  if depth == 0 {
    let positive = pos.cmpge(IVec4::ZERO);
    (positive.bitmask() as u8, pos.abs())
  } else {
    let one_bits = pos & 1;
    let one_bits_bvec = one_bits.cmpne(IVec4::ZERO);
    (one_bits_bvec.bitmask() as u8, pos >> 1)
  }
}

fn empty_branch() -> Box<[Option<TreeLevel>; 16]> {
  // ghughjkhfhg. but snazzy! makes None
  Default::default()
}

/// Avoid duplicating code on &mut and & accesses
/// Also, teach myself GATs.
trait HorribleIndexingGat {
  type TreeRef<'me>
  where
    Self: 'me;
  type FoxelRef<'me>
  where
    Self: 'me;

  fn index<'a>(
    tree: Self::TreeRef<'a>,
    idx: usize,
  ) -> Result<Option<Self::TreeRef<'a>>, Self::FoxelRef<'a>>;

  fn recurse_down<'a>(
    tree: Self::TreeRef<'a>,
    pos: IVec4,
    depth: usize,
  ) -> Option<Self::FoxelRef<'a>> {
    let (idx, pos2) = step_down_pos(pos, depth);

    let kiddo = Self::index(tree, idx as usize);
    if depth == Hexadecitree::DEPTH - 1 {
      // we're at the bottom!
      let Err(foxel) = kiddo else {
        panic!("tried to get foxels out of a branch node")
      };
      Some(foxel)
    } else {
      let Ok(tree2) = kiddo else {
        panic!("tried to get branches out of a leaf node")
      };
      let tree2 = tree2?;
      Self::recurse_down(tree2, pos2, depth + 1)
    }
  }
}

struct IdxRef;
struct IdxMut;

impl HorribleIndexingGat for IdxRef {
  type TreeRef<'me> = &'me TreeLevel;

  type FoxelRef<'me> = &'me Foxel;

  fn index<'a>(
    tree: Self::TreeRef<'a>,
    idx: usize,
  ) -> Result<Option<Self::TreeRef<'a>>, Self::FoxelRef<'a>> {
    match tree {
      TreeLevel::Branch(b) => Ok(b[idx].as_ref()),
      TreeLevel::Leaf(l) => Err(&l[idx]),
    }
  }
}

impl HorribleIndexingGat for IdxMut {
  type TreeRef<'me> = &'me mut TreeLevel;

  type FoxelRef<'me> = &'me mut Foxel;

  fn index<'a>(
    tree: Self::TreeRef<'a>,
    idx: usize,
  ) -> Result<Option<Self::TreeRef<'a>>, Self::FoxelRef<'a>> {
    match tree {
      TreeLevel::Branch(b) => Ok(b[idx].as_mut()),
      TreeLevel::Leaf(l) => Err(&mut l[idx]),
    }
  }
}
