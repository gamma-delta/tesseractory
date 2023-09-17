use crate::{
  math::{Axis, Rotor4},
  GameParams,
};
use getset::{CopyGetters, Getters};
use ultraviolet::{Rotor3, Vec4};

#[derive(Debug, CopyGetters, Getters)]
pub struct Player {
  #[getset(get_copy = "pub")]
  pos: Vec4,
  #[getset(get_copy = "pub")]
  look: Rotor4,
  #[getset(get_copy = "pub")]
  imag_axis: Axis,
}

impl Player {
  pub fn new(pos: Vec4) -> Self {
    let look = Rotor4::identity();
    Self {
      pos,
      look,
      imag_axis: Axis::W,
    }
  }

  pub fn physics_process(&mut self, delta: f32, cfg: &GameParams) {
    let input = godot::engine::Input::singleton();

    let dw = if input.is_action_just_pressed("imaginary_plus".into())
      || input.is_action_just_released("imaginary_plus_wheel".into())
    {
      Some(1)
    } else if input.is_action_just_pressed("imaginary_minus".into())
      || input.is_action_just_released("imaginary_minus_wheel".into())
    {
      Some(-1)
    } else {
      None
    };
    if let Some(dw) = dw {
      let idx = self.imag_axis as usize;
      self.pos[idx] += dw as f32;
    }

    let mut dx = 0.0;
    if input.is_action_pressed("up".into()) {
      dx += 1.0;
    }
    if input.is_action_pressed("down".into()) {
      dx -= 1.0;
    }
    self.pos.x += dx * cfg.player_fly_speed * delta;
  }
}
