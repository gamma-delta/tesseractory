use glam::IVec4;
use itertools::Itertools;

use crate::{
  math::Axis,
  type_aliases::{UnitVec4, Vec4},
};

/// https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.42.3443&rep=rep1&type=pdf
#[derive(Debug, Clone)]
pub struct AWFoxelIter {
  /// Each elt here is ±1, indicating if each axis is incremented or
  /// decremented at foxel boundaries
  steps: IVec4,
  /// Less a vector, more a float[4]. How many units of time one has to step
  /// in order to traverse one unit in that axis.
  t_delta: Vec4,

  /// What the paper just calls X, Y, Z
  cursor: IVec4,
  t_max: Vec4,
}

impl AWFoxelIter {
  pub fn new(origin: Vec4, heading: UnitVec4) -> Self {
    let steps = heading.into_iter().map(|n| n.signum() as i32).collect_vec();
    let steps = IVec4::from_slice(&steps);

    // we want 0 => inf here, but .inverse catches that
    let t_delta = heading.into_iter().map(|n| n.recip().abs()).collect_vec();
    let t_delta = Vec4::from_slice(&t_delta);

    let cursor = origin.into_iter().map(|n| n.floor() as i32).collect_vec();
    let cursor = IVec4::from_slice(&cursor);

    let t_max = heading
      .into_iter()
      .enumerate()
      .map(|(axis, head_val)| {
        if head_val == 0.0 {
          return f32::INFINITY;
        };

        let origin_val = origin[axis];
        let dist_to_wall = if steps[axis] > 0 {
          // Moving in the positive direction
          1.0 - origin_val.fract()
        } else {
          origin_val.fract()
        };

        dist_to_wall / head_val.abs()
      })
      .collect_vec();
    let t_max = Vec4::from_slice(&t_max);

    AWFoxelIter {
      steps,
      t_delta: t_delta.into_simple(),
      cursor,
      t_max,
    }
  }
}

impl Iterator for AWFoxelIter {
  type Item = AWFoxelIterElt;

  fn next(&mut self) -> Option<Self::Item> {
    let (min_axis, _) = self
      .t_max
      .as_slice()
      .iter()
      .copied()
      .enumerate()
      .min_by(|(_, a), (_, b)| a.total_cmp(&b))
      .unwrap();
    self.t_max[min_axis] += self.t_delta[min_axis];
    self.cursor[min_axis] += self.steps[min_axis];

    let normal_axis = Axis::try_from(min_axis as u8).unwrap();
    // If we are travelling in the positive direction, we are hitting the
    // negative faces
    let normal_positive = self.steps[min_axis] < 0;
    Some(AWFoxelIterElt {
      coord: self.cursor,
      normal_axis,
      normal_positive,
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AWFoxelIterElt {
  pub coord: IVec4,
  pub normal_axis: Axis,
  pub normal_positive: bool,
}

impl AWFoxelIterElt {
  pub fn normal(&self) -> IVec4 {
    let mut n = IVec4::ZERO;
    n[self.normal_axis as u8 as usize] =
      if self.normal_positive { 1 } else { -1 };
    n
  }
}

pub fn foxel_iter(start: Vec4, heading: UnitVec4) -> AWFoxelIter {
  AWFoxelIter::new(start, heading)
}
