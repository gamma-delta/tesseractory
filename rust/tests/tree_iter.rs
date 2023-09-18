use itertools::Itertools;
use tesseractory::math::{
  basis4,
  hexadecitree::iter::{IterItem, TreeIter},
  BlockPos,
};
use ultraviolet::{IVec4, Vec4};

/// Make sure that just going along a given axis works
#[test]
fn orthagonal() {
  let len = 100;
  for offset in [0.0, 0.5] {
    for axis in 0usize..=3 {
      let start = Vec4::broadcast(offset);
      let heading = basis4(axis);

      let line = TreeIter::new(start, heading).take(len).collect_vec();

      let hopeful_line = (0..len)
        .map(|n| {
          let mut v = IVec4::zero();
          v[axis] = n as i32 + 1;

          IterItem {
            pos: BlockPos(v),
            normal: basis4(axis) * -1.0,
          }
        })
        .collect_vec();
      if line != hopeful_line {
        panic!(
          "failed on axis {} : \ngot: {:?}\nexpected: {:?}\n",
          axis, line, hopeful_line
        )
      }
    }
  }
}

/// Make sure each foxel pos is next to the previous
#[test]
fn pbt_no_jumps() {
  let len = 100;

  let start = Vec4::new(0.7, -0.6, 0.0, -0.4);
  // make sure no corners are passed
  let heading = Vec4::new(1.1, 2.01, -3.02, 4.03).normalized();
  let line = TreeIter::new(start, heading)
    .map(|it| it.pos)
    .take(len)
    .collect_vec();

  for window in line.windows(2) {
    let &[a, b] = window else { unreachable!() };
    let diff = a.0 - b.0;
    if diff.mag_sq() != 1 {
      panic!("{:?} - {:?} = {:?}, not len 1", a, b, diff);
    }
  }
}

#[test]
fn respect_subfoxel() {
  let heading = Vec4::new(1.0, 1.0, 0.0, 0.0).normalized();

  let under_corner = TreeIter::new(Vec4::new(0.1, 0.0, 0.0, 0.0), heading)
    .next()
    .unwrap();
  let over_corner = TreeIter::new(Vec4::new(0.0, 0.1, 0.0, 0.0), heading)
    .next()
    .unwrap();

  // Printing like this so i can see both in the test output at the
  // same time because it's currently Borken
  assert_eq!(
    [under_corner.pos.0, over_corner.pos.0],
    [IVec4::new(1, 0, 0, 0), IVec4::new(0, 1, 0, 0)]
  );
}

#[test]
fn negative_headings() {
  let line =
    TreeIter::new(Vec4::zero(), Vec4::new(-1.0, -0.49, 0.0, 0.0).normalized())
      .take(10)
      .map(|hit| hit.pos.0)
      .collect_vec();
  assert_eq!(
    line,
    vec![
      IVec4::new(-1, 0, 0, 0),
      IVec4::new(-1, -1, 0, 0),
      IVec4::new(-2, -1, 0, 0),
      IVec4::new(-3, -1, 0, 0),
      IVec4::new(-3, -2, 0, 0),
      IVec4::new(-4, -2, 0, 0),
      IVec4::new(-5, -2, 0, 0),
      IVec4::new(-5, -3, 0, 0),
      IVec4::new(-6, -3, 0, 0),
      IVec4::new(-7, -3, 0, 0),
    ]
  )
}

#[test]
fn losing_my_mind_over_here() {
  let origin = Vec4::zero();
  let over_heading = Vec4::new(0.001, 1.0, 0.0, 0.0).normalized();
  let under_heading = Vec4::new(-0.001, 1.0, 0.0, 0.0).normalized();

  let over_line = TreeIter::new(origin, over_heading)
    .take(10)
    .map(|it| it.pos)
    .collect_vec();
  let under_line = TreeIter::new(origin, under_heading)
    .take(10)
    .map(|it| it.pos)
    .collect_vec();

  let hope_over = (1..=10).map(|y| BlockPos::new(0, y, 0, 0)).collect_vec();
  let hope_under = (1..=10).map(|y| BlockPos::new(-1, y, 0, 0)).collect_vec();

  assert_eq!(over_line, hope_over);
  assert_eq!(under_line, hope_under);
}
