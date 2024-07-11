use tesseractory::extensions::F32Ext;
use ultraviolet::{Vec2, Vec3};

fn brick_size_for_pos(v: Vec2) -> f32 {
  let range = 4.0..8.0;
  // if range.contains(&v.x) && range.contains(&v.y) {
  //   4.0
  // } else {
  //   1.0
  // }
}

#[test]
pub fn dda_augh() {
  // https://www.cs.cornell.edu/courses/cs4620/2013fa/lectures/03raytracing1.pdf
  // It's just boxes!
  let start = Vec2::new(2.4, 3.5);
  let direction = Vec2::new(-1.0, -1.5).normalized();

  let step_sizes = direction.abs().map(f32::recip);
  let step_signs = direction.map(F32Ext::good_sign);

  let mut curr_pos = start;

  println!("{},{}", curr_pos.x, curr_pos.y);
  for _ in 0..16 {
    let brick_size = brick_size_for_pos(curr_pos);
    let voxel_ends =
      ((curr_pos / brick_size).map(f32::floor) + step_signs) * brick_size;
    let t_mins = ((voxel_ends - curr_pos) / direction).abs();
    let smallest_t = t_mins.component_min();

    curr_pos += direction * smallest_t;

    let foxel_pos = (curr_pos + direction * 0.0001).map(f32::floor);
    println!("{},{}", curr_pos.x, curr_pos.y,);
  }
}
