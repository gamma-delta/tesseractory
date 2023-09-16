pub mod algos;
pub mod extensions;
pub mod godot_bridge;
pub mod math;
pub mod player;
pub mod type_aliases;
pub mod world;

use extensions::GodotObjectExt;
use glam::IVec2;
use godot::prelude::{Gd, Input, RefCounted, Resource};
use math::BlockPos;
use player::Player;
use type_aliases::GVec2;
use world::{Foxel, World};

use crate::type_aliases::{Color, Vec4};

pub struct GameState {
  world: World,
  player: Player,
  canvas_size: IVec2,

  params: GameParams,
}

impl GameState {
  pub fn new(canvas_size: IVec2, params: GameParams) -> Self {
    let sun_dir = Vec4::new(-0.5, 0.4, 0.2, 0.0).normalize();
    let mut world = World::new(sun_dir);
    world.setup_sample_scene();

    let player = Player::new(Vec4::new(0.0, -3.0, 0.0, 0.5));

    Self {
      world,
      player,
      canvas_size,
      params,
    }
  }

  pub fn physics_process(&mut self, delta: f32) {
    self.player.physics_process(delta, &self.params);
  }

  pub fn draw_world(&self, canvas: &mut Vec<[u8; 3]>) {
    use rayon::prelude::*;

    let len = self.canvas_size.y * self.canvas_size.x;
    (0..len)
      .into_par_iter()
      .map(|i| {
        let x = i % self.canvas_size.x;
        let y = i / self.canvas_size.x;

        let px = IVec2::new(x as _, y as _);
        let ray = self.world_ray(px);

        let iter = algos::foxel_iter(self.player.pos(), ray.normalize());
        let hit = iter.take(10).find_map(|hit| {
          let foxel = self.world.get_foxel(BlockPos(hit.coord))?;
          (foxel != Foxel::Air).then_some((hit, foxel))
        });
        let color = if let Some((hit, foxel)) = hit {
          let col = foxel.color();

          // finally actually using any geometric algebra
          let norm_dot =
            hit.normal.into_inner() % -self.world.sun_dir().into_inner();
          let normal_light = norm_dot[0].clamp(0.0, 1.0);
          let ambient_light = 0.5;
          col * (normal_light * 0.5 + ambient_light).clamp(0.0, 1.0)
        } else {
          Color::from_rgb(0.2, 0.2, 0.2)
        };
        [color.r8(), color.g8(), color.b8()]
      })
      .collect_into_vec(canvas);
  }

  pub fn debug_info(&self) -> String {
    let pos = self.player.pos();
    let imag = self.player.imag_axis();
    let fps = godot::engine::Engine::singleton().get_frames_per_second();
    format!("pos: {pos}\nimag: {imag:?}\nfps: {fps:.4}")
  }

  fn world_ray(&self, px: IVec2) -> Vec4 {
    let centered = px - self.canvas_size / 2;
    let centered = GVec2::new(centered.x as _, centered.y as _);
    let in_2d = centered * GVec2::splat(self.params.fov);

    let offset = Vec4::new(-in_2d.y, self.params.focal_dist, -in_2d.x, 0.0);

    self.player.look().rot(offset)
  }
}

pub struct GameParams {
  pub focal_dist: f32,
  pub fov: f32,
  pub player_walk_speed: f32,
  pub player_fly_speed: f32,
}

impl GameParams {
  pub fn load(cfg: &Gd<Resource>) -> Self {
    Self {
      focal_dist: cfg.totally::<f32>("focal_dist") / 1_000.0,
      fov: cfg.totally::<f32>("fov") / 10_000.0,
      player_walk_speed: cfg.totally("walk_speed"),
      player_fly_speed: cfg.totally("fly_speed"),
    }
  }
}
