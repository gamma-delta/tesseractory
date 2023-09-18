mod bivec4;
mod rotor4;

pub use bivec4::*;
pub use rotor4::*;

/// Re-implementation of uv's splat trait
trait Splat<T> {
  fn splat(val: T) -> Self;
}

impl Splat<f32> for f32 {
  #[inline(always)]
  fn splat(val: f32) -> Self {
    val
  }
}
