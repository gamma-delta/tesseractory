use tesseractory::extensions::F32Ext;
use ultraviolet::{Vec2, Vec3};

fn is_in_skip_brick(v: Vec2) -> bool {
  v.as_array().into_iter().all(|n| (4.0..=8.0).contains(n))
}

#[test]
pub fn dda_augh() {
  let start = Vec2::new(2.4, 3.5);
  let direction = Vec2::new(1.0, 1.5).normalized();

  let step_sizes = direction.abs().map(f32::recip);
  let step_dir = direction.map(F32Ext::good_sign);

  let mut curr_pos = start;
  let mut next_dist =
    (step_dir * 0.5 + Vec2::broadcast(0.5) - start.map(f32::fract)) / direction;
  let mut foxel_pos = start.map(f32::floor);

  println!("{},{}", curr_pos.x, curr_pos.y);
  for i in 0..16 {
    let stride = if is_in_skip_brick(curr_pos) { 4.0 } else { 1.0 };

    let closest_dist = next_dist.component_min();
    let step_axis = next_dist.map(|n| {
      if n <= closest_dist + 0.000001 {
        1.0
      } else {
        0.0
      }
    });

    curr_pos += direction * closest_dist * stride;
    // foxel_pos += step_axis * step_dir;
    next_dist +=
      Vec2::broadcast(-closest_dist) + step_sizes * step_axis * stride;

    println!("{},{}", curr_pos.x, curr_pos.y);
  }
}
