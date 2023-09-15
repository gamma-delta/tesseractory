pub mod algos;
pub mod godot_bridge;
pub mod math;
pub mod type_aliases;
pub mod world;

use getset::CopyGetters;
use glam::IVec2;
use godot::prelude::Input;
use godot_bridge::GodotEditorConfig;
use math::{Axis, BlockPos};
use type_aliases::{GMat4, GVec2, Rotor4};
use wedged::base::One;
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
    world.setup_sample_scene();
    Self { camera, world }
  }

  pub fn update(&mut self, cfg: GodotEditorConfig, delta: f32) {
    self.camera.fov = cfg.fov;
    self.camera.focal_dist = cfg.focal_dist;

    let input = Input::singleton();

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
      let idx = self.camera.imag_axis as usize;
      self.camera.raw_pos[idx] += dw as f32;
    }

    let mut dx = 0.0;
    if input.is_action_pressed("up".into()) {
      dx += 1.0;
    }
    if input.is_action_pressed("down".into()) {
      dx -= 1.0;
    }
    self.camera.raw_pos.x += dx * 5.0 * delta;
  }

  pub fn draw_world(&self, mut canvas: CanvasWrapper<'_>) {
    let lightdir = GVec4::new(-0.5, 0.4, 0.2, 0.0).normalize();

    for y in 0..self.camera.canvas_y {
      for x in 0..self.camera.canvas_x {
        let px = IVec2::new(x as _, y as _);
        let ray = self.camera.world_ray(px);

        let iter = algos::foxel_iter(self.camera.pos(), ray.normalize());
        let hit = iter.take(10).find_map(|hit| {
          let foxel = self.world.foxel_at(BlockPos(hit.coord))?;
          (foxel != FoxelType::Air).then_some((hit, foxel))
        });
        let color = if let Some((hit, foxel)) = hit {
          let col = foxel.color();
          let normal_light =
            hit.normal().as_vec4().dot(-lightdir).clamp(0.0, 1.0);
          let ambient_light = 0.5;
          col * (normal_light * 0.5 + ambient_light).clamp(0.0, 1.0)
        } else {
          Color::from_rgb(0.2, 0.2, 0.2)
        };

        canvas.set_pixel(px, color);
      }
    }
  }

  pub fn debug_info(&self) -> String {
    let pos = self.camera.pos();
    let imag = self.camera.imag_axis;
    format!("pos: {pos}\nimag: {imag:?}")
  }
}

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Camera {
  /// this keeps the imaginary axis at integer coords
  raw_pos: Vec4,
  imag_axis: Axis,

  focal_dist: f32,
  fov: f32,

  canvas_x: u32,
  canvas_y: u32,
}

impl Camera {
  pub fn new(canvas_x: u32, canvas_y: u32) -> Self {
    Self {
      raw_pos: Vec4::new(1.0, -3.0, 0.0, 0.0),
      imag_axis: Axis::W,

      focal_dist: 0.007,
      fov: 0.00005,

      canvas_x,
      canvas_y,
    }
  }

  pub fn world_ray(&self, px: IVec2) -> Vec4 {
    let centered =
      px - IVec2::new(self.canvas_x as i32 / 2, self.canvas_y as i32 / 2);
    let centered = GVec2::new(centered.x as _, centered.y as _);
    let in_2d = centered * GVec2::new(self.fov, self.fov);

    let offset = Vec4::new(-in_2d.y, self.focal_dist, -in_2d.x, 0.0);

    offset
  }

  /// Get the position, with the imaginary directin always being an int + 0.5
  pub fn pos(&self) -> Vec4 {
    let mut p = self.raw_pos;
    p[self.imag_axis as usize] += 0.5;
    p
  }
}
