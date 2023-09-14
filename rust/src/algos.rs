use glam::IVec4;
use itertools::Itertools;

use crate::math::{UnitVec4, Vec4};

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
    let t_delta = heading.into_iter().map(f32::recip).collect_vec();
    let t_delta = Vec4::from_slice(&t_delta);

    let cursor = origin.into_iter().map(|n| n.round() as i32).collect_vec();
    let cursor = IVec4::from_slice(&cursor);

    // t_max[x] = min(1 - y.fract(), 1 - z.fract(), 1 - w.fract()) / x
    // and ditto for the other 3 axes
    // to avoid typing hell, i have written it like this below
    // which is a lot more complicated but less horrible to maintain
    let t_max = heading
      .into_iter()
      .enumerate()
      .map(|(axis, head_val)| {
        use std::cmp::Ordering;

        let subpx = origin[axis].fract();
        let dist = match head_val.total_cmp(&0.0) {
          // The heading is perpendicular to this axis, so it will never
          // breach this hface. so return infinity
          Ordering::Equal => return f32::INFINITY,
          Ordering::Less => -subpx,
          Ordering::Greater => 1.0 - subpx,
        };
        dist / head_val.abs()
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
    dbg!(self.t_max);
    self.t_max[min_axis] += self.t_delta[min_axis];
    self.cursor[min_axis] += self.steps[min_axis];
    Some(AWFoxelIterElt { coord: self.cursor })
  }
}

#[derive(Debug, Clone, Copy)]
pub struct AWFoxelIterElt {
  pub coord: IVec4,
  // TODO: probably get the hit pos on the hyperface
}

pub fn foxel_iter(start: Vec4, heading: UnitVec4) -> AWFoxelIter {
  AWFoxelIter::new(start, heading)
}
