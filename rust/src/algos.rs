use itertools::Itertools;
use ultraviolet::{IVec4, Vec4};

use crate::math::basis4;

/// https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.42.3443&rep=rep1&type=pdf
#[derive(Debug, Clone)]
pub struct AWFoxelIter {
  /// Each elt here is ±1, indicating if each axis is incremented or
  /// decremented at foxel boundaries
  steps: [i32; 4],
  /// Less a vector, more a float[4]. How many units of time one has to step
  /// in order to traverse one unit in that axis.
  t_delta: [f32; 4],

  /// What the paper just calls X, Y, Z
  cursor: IVec4,
  t_max: [f32; 4],
}

impl AWFoxelIter {
  pub fn new(origin: Vec4, heading: Vec4) -> Self {
    let steps = heading.as_array().map(|n| n.signum() as i32);

    // we want 0 => inf here
    let t_delta = heading.as_array().map(|n| n.recip().abs());

    let cursor = origin.as_array().map(|n| n.floor() as i32);
    let cursor = IVec4::from(cursor);

    let t_max = heading
      .as_array()
      .into_iter()
      .enumerate()
      .map(|(axis, &head_val)| {
        if head_val == 0.0 {
          return f32::INFINITY;
        };

        let origin_val = origin[axis];
        let real_wall_pos = if head_val > 0.0 {
          origin_val.floor() + 1.0
        } else {
          origin_val.ceil() - 1.0
        };
        let dist_to_wall = origin_val - real_wall_pos;

        dist_to_wall.abs() / head_val.abs()
      })
      .collect_vec();
    let t_max = t_max.try_into().unwrap();

    AWFoxelIter {
      steps,
      t_delta,
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
      .into_iter()
      .enumerate()
      .min_by(|(_, a), (_, b)| a.total_cmp(&b))
      .unwrap();
    self.t_max[min_axis] += self.t_delta[min_axis];
    self.cursor[min_axis] += self.steps[min_axis];

    // If we are travelling in the positive direction, we are hitting the
    // negative faces
    let normal = basis4(min_axis) * -self.steps[min_axis] as f32;
    Some(AWFoxelIterElt {
      coord: self.cursor,
      normal,
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AWFoxelIterElt {
  pub coord: IVec4,
  pub normal: Vec4,
}

pub fn foxel_iter(start: Vec4, heading: Vec4) -> AWFoxelIter {
  AWFoxelIter::new(start, heading.normalized())
}
