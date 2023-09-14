pub mod algos;
pub mod godot_bridge;
pub mod math;
pub mod world;

use getset::CopyGetters;
use glam::IVec2;
use math::GVec2;
use wedged::{
  base::{Const, DimNameAdd},
  subspace::SimpleBlade,
};

use crate::math::{Axis, GVec3, GVec4, Vec3, Vec4};

pub struct GameState {
  camera: Camera,
}

impl GameState {
  pub fn new(camera: Camera) -> Self {
    Self { camera }
  }
}

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Camera {
  pos_real: Vec3,
  imag: i32,

  focal_dist: f32,
  fovx: f32,
  fovy: f32,

  canvas_x: u32,
  canvas_y: u32,
}

impl Camera {
  pub fn new(canvas_x: u32, canvas_y: u32) -> Self {
    Self {
      pos_real: Vec3::zeroed(),
      imag: 0,
      focal_dist: 0.01,
      fovx: 0.01,
      fovy: 0.01,

      canvas_x,
      canvas_y,
    }
  }

  pub fn worldspace_px(&self, px: IVec2) -> Vec4 {
    let centered =
      px - IVec2::new(self.canvas_x as i32 / 2, self.canvas_y as i32 / 2);
    let centered = GVec2::new(centered.x as _, centered.y as _);
    let in_2d = centered * GVec2::new(self.fovx, self.fovy);

    let offset = Vec4::new(-in_2d.y, self.focal_dist, -in_2d.x, 0.0);
    self.pos() + offset
  }

  pub fn pos(&self) -> Vec4 {
    let mut updim: Vec4 = self.pos_real.cast_dim();
    updim.w = self.imag as _;
    updim
  }
}
