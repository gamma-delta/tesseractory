pub mod algos;
pub mod godot_bridge;
pub mod math;
pub mod type_aliases;
pub mod world;

use getset::CopyGetters;
use glam::IVec2;
use math::BlockPos;
use type_aliases::GVec2;
use world::FoxelType;

use crate::{
  godot_bridge::CanvasWrapper,
  type_aliases::{Color, GVec3, GVec4, UnitVec3, Vec3, Vec4},
  world::FoxelStore,
};

pub struct GameState {
  camera: Camera,
  world: FoxelStore,
}

impl GameState {
  pub fn new(camera: Camera) -> Self {
    let mut world = FoxelStore::new();
    world.sample_scene();
    Self { camera, world }
  }

  pub fn draw_world(&self, mut canvas: CanvasWrapper<'_>) {
    for y in 0..self.camera.canvas_y {
      for x in 0..self.camera.canvas_x {
        let px = IVec2::new(x as _, y as _);
        let worldpx = self.camera.worldspace_px(px);
        let ray = worldpx - self.camera.pos();

        let iter = algos::foxel_iter(worldpx, ray.normalize());
        let hit = iter.take(10).find_map(|pos| {
          let foxel = self.world.foxel_at(BlockPos(pos.coord))?;
          (foxel != FoxelType::Air).then_some(foxel)
        });
        let color = if let Some(hit) = hit {
          hit.color()
        } else {
          Color::from_rgb(0.2, 0.2, 0.2)
        };

        canvas.set_pixel(px, color);
      }
    }
  }
}

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Camera {
  pos_real: Vec3,
  pos_imag: i32,
  heading_real: UnitVec3,

  focal_dist: f32,
  fovx: f32,
  fovy: f32,

  canvas_x: u32,
  canvas_y: u32,
}

impl Camera {
  pub fn new(canvas_x: u32, canvas_y: u32) -> Self {
    Self {
      pos_real: Vec3::new(2.0, 0.0, 0.0),
      pos_imag: 0,
      heading_real: Vec3::new(0.0, 1.0, 0.0).normalize(),

      focal_dist: 0.01,
      fovx: 0.0001,
      fovy: 0.0001,

      canvas_x,
      canvas_y,
    }
  }

  pub fn worldspace_px(&self, px: IVec2) -> Vec4 {
    let centered =
      px - IVec2::new(self.canvas_x as i32 / 2, self.canvas_y as i32 / 2);
    let centered = GVec2::new(centered.x as _, centered.y as _);
    // let aspect = self.canvas_x as f32 / self.canvas_y as f32;
    let in_2d = centered * GVec2::new(self.fovx, self.fovy);

    let offset = Vec4::new(-in_2d.y, self.focal_dist, -in_2d.x, 0.0);
    self.pos() + offset
  }

  pub fn pos(&self) -> Vec4 {
    // what's updim
    let mut updim: Vec4 = self.pos_real.cast_dim();
    // not much what's updim double-you?
    updim.w = self.pos_imag as _;
    updim
  }
}
