use itertools::Itertools;
use ultraviolet::{IVec4, Vec4};

use crate::{extensions::F32Ext, math::BlockPos};

pub struct TreeIter {
  // Optimizations to save on recomputing ray coeffs
  dir_recip: Vec4,
  slope_something: Vec4,

  cursor: IVec4,
  signums: IVec4,
}

impl TreeIter {
  pub fn new(start: Vec4, dir: Vec4) -> Self {
    let signums = dir.as_array().map(|v| v.good_sign() as i32);
    let signums = IVec4::from(signums);

    let mut cursor = IVec4::zero();
    for axis in 0..4 {
      // Flooring always goes to the smallest xyzw
      // But, we want the start cursor point to be the corner furthest
      // from the ray path.
      // This way the cursor starts surrounding the start pos.
      // If the ray is going positive, then this is all fine, because the
      // cursor covers from behind it to in front of it ...
      // but if it's going negative it has to swap.
      //
      // I'm not sure how to explain this in a comment, it took me like
      // half an hour of scribbling on graph paper to figure this one out
      let axis_aligned_origin = start[axis].fract() == 0.0;
      let offset = if !axis_aligned_origin && signums[axis] < 0 {
        1
      } else {
        0
      };
      cursor[axis] = start[axis].floor() as i32 + offset;
    }

    let dir_recip = Vec4::one() / dir;
    let slope_something = -start * dir_recip;

    Self {
      dir_recip,
      slope_something,
      cursor,
      signums,
    }
  }
}

impl Iterator for TreeIter {
  type Item = IterItem;

  fn next(&mut self) -> Option<Self::Item> {
    // what the paper calls {xyz}_1.
    // the position of the other corner of the cube.
    let exit_poses = Vec4::from(self.cursor + self.signums);
    let exit_times = exit_poses.mul_add(self.dir_recip, self.slope_something);
    let exit_time = exit_times.component_min();
    // ugh
    let min_time_map = exit_times
      .as_array()
      .map(|v| ((v - exit_time).abs() < 0.00001) as i32);
    let min_time_map = IVec4::from(min_time_map);
    let cursor_inc = min_time_map * self.signums;

    self.cursor += cursor_inc;

    Some(IterItem {
      pos: BlockPos(self.cursor),
      normal: -Vec4::from(cursor_inc),
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IterItem {
  pub pos: BlockPos,
  pub normal: Vec4,
}
