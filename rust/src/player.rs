use std::f32::consts::TAU;

use crate::{
  math::{
    geo::{Bivec4, Rotor4},
    Axis,
  },
  GameParams,
};
use getset::{CopyGetters, Getters};
use godot::prelude::Input;
use ultraviolet::Vec4;

#[derive(Debug, CopyGetters, Getters)]
pub struct Player {
  #[getset(get_copy = "pub")]
  pos: Vec4,

  camera: AxisCamera,
}

impl Player {
  pub fn new(pos: Vec4) -> Self {
    Self {
      pos,
      camera: AxisCamera::new(),
    }
  }

  pub fn look(&self) -> Rotor4 {
    self.camera.rotor()
  }

  pub fn physics_process(&mut self, delta: f32, cfg: &GameParams) {
    self.camera.update_from_ctrl(delta, &cfg);

    let dv = Player::delta_move(delta, cfg);
    let tfed_move = self.camera.transform_movement(dv);
    self.pos += tfed_move;
  }

  fn delta_move(delta: f32, cfg: &GameParams) -> Vec4 {
    let input = godot::engine::Input::singleton();
    let mut dv = Vec4::zero();

    if input.is_action_pressed("up".into()) {
      dv.x += 1.0 * cfg.player_fly_speed;
    }
    if input.is_action_pressed("down".into()) {
      dv.x -= 1.0 * cfg.player_fly_speed;
    }
    if input.is_action_pressed("forward".into()) {
      dv.y += 1.0 * cfg.player_walk_speed;
    }
    if input.is_action_pressed("back".into()) {
      dv.y -= 1.0 * cfg.player_walk_speed;
    }
    if input.is_action_pressed("left".into()) {
      dv.z += 1.0 * cfg.player_walk_speed;
    }
    if input.is_action_pressed("right".into()) {
      dv.z -= 1.0 * cfg.player_walk_speed;
    }

    dv *= delta;

    // W coordinates we don't want to use the delta
    let plus = input.is_action_just_pressed("imaginary_plus".into())
      || input.is_action_just_released("imaginary_plus_wheel".into());
    let minus = input.is_action_just_pressed("imaginary_minus".into())
      || input.is_action_just_released("imaginary_minus_wheel".into());
    if plus && !input.is_action_just_released("imaginary_turn_plus".into()) {
      dv.w += 1.0;
    } else if minus
      && !input.is_action_just_released("imaginary_turn_minus".into())
    {
      dv.w -= 1.0;
    };

    dv
  }

  pub fn debug_info(&self, s: &mut String) {
    let Vec4 { x, y, z, w } = self.pos;
    *s += &format!("X/Y/Z/W: {x} / {y} / {z} / {w}\n");
    self.camera.debug_info(s);
  }
}

#[derive(Debug)]
struct AxisCamera {
  /// Local YZ plane rotation
  rot_yz: f32,
  /// Local XY plane rotation
  rot_xy: f32,
  imag_axis: Axis,
}

impl AxisCamera {
  pub fn new() -> Self {
    Self {
      rot_yz: 0.0,
      rot_xy: 0.0,
      imag_axis: Axis::W,
    }
  }

  pub fn rotor(&self) -> Rotor4 {
    let w2imag =
      Rotor4::from_rotation_between(Axis::W.basis(), self.imag_axis.basis());
    let local_yz = Rotor4::from_angle_plane(self.rot_yz, Bivec4::unit_yz());
    let local_xy = Rotor4::from_angle_plane(self.rot_xy, Bivec4::unit_xy());
    w2imag * local_yz * local_xy
  }

  /// Input is local coordinates. Again, X is up, Y is forward, Z is left.
  /// W is imaginary, for the scroll wheel
  pub fn transform_movement(&self, movement: Vec4) -> Vec4 {
    // If we're looking up or down we still want flight to move up.
    let mut raw_move = self.rotor() * movement;
    raw_move.x = movement.x;
    raw_move
  }

  pub fn update_from_ctrl(&mut self, delta: f32, params: &GameParams) {
    let mut input = Input::singleton();

    let mouse = input.get_last_mouse_velocity();
    let d_yz = -mouse.x * params.look_speed * delta;
    let d_xy = mouse.y * params.look_speed * delta;

    self.rot_yz += d_yz;
    self.rot_yz = self.rot_yz.rem_euclid(TAU);

    self.rot_xy += d_xy;
    self.rot_xy = self.rot_xy.clamp(-TAU / 4.0, TAU / 4.0);

    let d_imag = if input.is_action_just_released("imaginary_turn_plus".into())
    {
      Some(1)
    } else if input.is_action_just_released("imaginary_turn_minus".into()) {
      Some(-1)
    } else {
      None
    };
    if let Some(d_imag) = d_imag {
      let current_idx = self.imag_axis as i8;
      // This skips X
      let idx = (current_idx - 1 + d_imag).rem_euclid(3) + 1;
      self.imag_axis = Axis::try_from(idx as u8).unwrap();
    }
  }

  pub fn debug_info(&self, w: &mut String) {
    *w += "Camera mode: AxisCamera\n";
    *w += &format!("  Local rot YZ/XY : {} / {}\n", self.rot_yz, self.rot_xy);
    *w += &format!("  Imag: {:?}\n", self.imag_axis);
  }
}
