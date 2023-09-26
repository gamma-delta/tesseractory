//! GPU-friendly representations of stuff

use bytemuck::NoUninit;

use super::Hexadecitree;

const HIGH_BIT: u32 = 1 << 31;

#[derive(Debug)]
pub(super) enum TreeLevel {
  Empty,
  /// Indices into the branch arena.
  /// Index this by going to the tree ref, then adding the child pointer.
  Branch(TreeRef),
  Leaf(FoxelSpanRef),
}

impl TreeLevel {
  pub fn enc(&self) -> TreeLevelInner {
    TreeLevelInner(match self {
      TreeLevel::Empty => 0,
      TreeLevel::Branch(TreeRef(x)) => {
        debug_assert_eq!(x & HIGH_BIT, 0);
        x & (!HIGH_BIT)
      }
      &TreeLevel::Leaf(FoxelSpanRef(x)) => HIGH_BIT | x as u32,
    })
  }
}

/// Compressed version of TreeLevel.
///
/// - `0b00000000...` : `TreeLevel::Empty`
/// - `0b0XXXXXXX...` : `TreeLevel::Branch`
/// - `0b1XXXXXXX...` : `TreeLevel::Leaf` (although the high bits are ignored)
#[derive(Debug, Clone, Copy, NoUninit)]
#[repr(transparent)]
pub(super) struct TreeLevelInner(u32);

impl TreeLevelInner {
  pub fn friendly(self) -> TreeLevel {
    let x = self.0;

    // The root node will never be stored in the tree, so we can use
    // 0 for empty
    if x == 0 {
      TreeLevel::Empty
    } else if (x & HIGH_BIT) == 0 {
      let unmasked = x & !HIGH_BIT;
      TreeLevel::Branch(TreeRef(unmasked))
    } else {
      // the high bit MUST be set.
      let squished = (x & 0xFFFF) as u16;
      TreeLevel::Leaf(FoxelSpanRef(squished))
    }
  }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(super) struct TreeRef(u32);

impl TreeRef {
  pub fn root() -> Self {
    Self(0)
  }

  pub fn child_idx(self, child_idx: u8) -> usize {
    self.0 as usize + child_idx as usize
  }

  pub fn from_idx(x: usize) -> Self {
    Self(x as u32)
  }

  pub fn idx(self) -> usize {
    self.0 as _
  }
}

impl std::fmt::Debug for TreeRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FoxelSpanRef(u16);

impl FoxelSpanRef {
  pub const SPAN: usize = Hexadecitree::CHILDREN;

  pub fn from_idx(x: usize) -> Self {
    debug_assert_eq!(x % Self::SPAN, 0);
    Self((x / Self::SPAN) as _)
  }

  pub fn idx(self, child_idx: u8) -> usize {
    self.0 as usize * Self::SPAN + child_idx as usize
  }
}
