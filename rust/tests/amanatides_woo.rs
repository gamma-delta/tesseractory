use glam::IVec4;
use itertools::Itertools;
use tesseractory::{
  algos::{self, AWFoxelIterElt},
  math::Axis,
  type_aliases::{UnitVec4, Vec4},
};
use wedged::base::Const;

/// Make sure that just going along a given axis works
#[test]
fn orthagonal() {
  let len = 100;
  for offset in [0.0, 0.5] {
    for axis in 0usize..=3 {
      let start = Vec4::new(offset, offset, offset, offset);
      let heading = UnitVec4::basis_generic(Const, Const, axis);

      let line = algos::foxel_iter(start, heading).take(len).collect_vec();

      let hopeful_line = (0..len)
        .map(|n| {
          let mut v = IVec4::ZERO;
          v[axis] = n as i32 + 1;

          AWFoxelIterElt {
            coord: v,
            normal: (Vec4::basis(axis) * -1.0).normalize(),
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
  let heading = Vec4::new(1.0, 2.0, -3.0, 4.0).normalize();
  let line = algos::foxel_iter(start, heading)
    .map(|it| it.coord)
    .take(len)
    .collect_vec();

  for window in line.windows(2) {
    let &[a, b] = window else { unreachable!() };
    let diff = a - b;
    if diff.length_squared() != 1 {
      panic!("{} - {} = {}, not len 1", a, b, diff);
    }
  }
}

#[test]
fn respect_subfoxel() {
  let heading = Vec4::new(1.0, 1.0, 0.0, 0.0).normalize();

  let under_corner = algos::foxel_iter(Vec4::new(0.1, 0.0, 0.0, 0.0), heading)
    .next()
    .unwrap();
  let over_corner = algos::foxel_iter(Vec4::new(0.0, 0.1, 0.0, 0.0), heading)
    .next()
    .unwrap();

  // Printing like this so i can see both in the test output at the
  // same time because it's currently Borken
  assert_eq!(
    [under_corner.coord, over_corner.coord],
    [IVec4::new(1, 0, 0, 0), IVec4::new(0, 1, 0, 0)]
  );
}

#[test]
fn negative_headings() {
  let line = algos::foxel_iter(
    Vec4::zeroed(),
    Vec4::new(-1.0, -0.45, 0.0, 0.0).normalize(),
  )
  .take(10)
  .map(|hit| hit.coord)
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
