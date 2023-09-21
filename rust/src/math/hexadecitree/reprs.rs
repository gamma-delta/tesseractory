//! GPU-friendly representations of stuff

/*
pub(super) enum TreeLevel {
  Empty,
  /// Indices into the branch arena.
  /// Index this by going to the tree ref, then adding the child pointer.
  Branch(TreeRef),
  Leaf(FoxelSpanRef),
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(super) struct TreeLevelInner(u32);

impl TreeLevelInner {
    pub fn friendly(self)     -> TreeLevel {
        let x = self.0;

        if x == 0 {
            TreeLevel::Empty
        } else if {

        }
    }
}
*/

use super::Hexadecitree;

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
