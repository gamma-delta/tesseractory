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
      let v = if dir[axis] > 0.0 {
        start[axis].floor()
      } else {
        start[axis].ceil()
      };
      cursor[axis] = v as i32;
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
