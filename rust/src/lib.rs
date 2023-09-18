pub mod extensions;
pub mod godot_bridge;
pub mod math;
pub mod player;
pub mod world;

use extensions::GodotObjectExt;
use godot::prelude::{Color, Gd, Resource};
use math::geo::Rotor4x8;
use player::Player;
use ultraviolet::{f32x8, IVec2, Vec2, Vec2x8, Vec4, Vec4x8};
use world::{Foxel, World};

use crate::math::hexadecitree::iter::TreeIter;

pub struct WorldState {
  world: World,
  player: Player,
  canvas_size: IVec2,

  params: GameParams,
}

impl WorldState {
  pub fn new(canvas_size: IVec2, params: GameParams) -> Self {
    let sun_dir = Vec4::new(-0.5, 0.4, 0.2, 0.1).normalized();
    let mut world = World::new(sun_dir);
    world.setup_sample_scene();

    let player = Player::new(Vec4::new(0.0, -3.0, 0.001, 0.5));

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

    let counting = f32x8::new([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    let looks = Rotor4x8::splat(self.player.look());

    let len = self.canvas_size.y * self.canvas_size.x;
    let chunked_len = len / 8;
    debug_assert_eq!(self.canvas_size.y % 8, 0);

    let mut pxs = Vec::new();
    (0..chunked_len)
      .into_par_iter()
      .map(|ci| {
        let i = ci * 8;
        let y = i / self.canvas_size.x;
        let base_x = i % self.canvas_size.x;
        let xs = f32x8::splat(base_x as f32) + counting;
        Vec2x8::new(xs, f32x8::splat(y as f32))
      })
      .collect_into_vec(&mut pxs);
    let tfed_pxes8 = self.world_rays(&looks, &pxs);

    let tfed_pxes = tfed_pxes8
      .into_par_iter()
      .flat_map_iter(|ray8| transpose_rays(ray8).into_iter())
      .collect::<Vec<_>>();

    tfed_pxes
      .into_par_iter()
      .map(|ray| {
        let iter = TreeIter::new(self.player.pos(), ray);
        let hit = iter.take(10).find_map(|hit| {
          let foxel = self.world.get_foxel(hit.pos)?;
          (foxel != Foxel::Air).then_some((hit, foxel))
        });
        let color = if let Some((hit, foxel)) = hit {
          let col = foxel.color();

          let norm_dot = hit.normal.dot(-self.world.sun_dir());
          let normal_light = norm_dot.clamp(0.0, 1.0);
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
    format!("pos: {pos:?}\nimag: {imag:?}\nfps: {fps:.4}")
  }

  fn world_rays(&self, looks: &Rotor4x8, pxs: &[Vec2x8]) -> Vec<Vec4x8> {
    use rayon::prelude::*;

    let half_canvas = Vec2::from(self.canvas_size / 2);
    let half_canvas8 = Vec2x8::splat(half_canvas);

    let focal = f32x8::splat(self.params.focal_dist);

    let mut world_poses = pxs
      .into_par_iter()
      .map(|&px| {
        let centered = px - half_canvas8;
        let in_2d = centered * f32x8::splat(self.params.fov);

        Vec4x8::new(-in_2d.y, focal, -in_2d.x, f32x8::ZERO)
      })
      .collect::<Vec<_>>();

    looks.rot_many(&mut world_poses);
    world_poses
  }
}

fn transpose_rays(ray8: Vec4x8) -> [Vec4; 8] {
  let xs = ray8.x.as_array_ref();
  let ys = ray8.y.as_array_ref();
  let zs = ray8.z.as_array_ref();
  let ws = ray8.w.as_array_ref();
  [
    Vec4::new(xs[0], ys[0], zs[0], ws[0]),
    Vec4::new(xs[1], ys[1], zs[1], ws[1]),
    Vec4::new(xs[2], ys[2], zs[2], ws[2]),
    Vec4::new(xs[3], ys[3], zs[3], ws[3]),
    Vec4::new(xs[4], ys[4], zs[4], ws[4]),
    Vec4::new(xs[5], ys[5], zs[5], ws[5]),
    Vec4::new(xs[6], ys[6], zs[6], ws[6]),
    Vec4::new(xs[7], ys[7], zs[7], ws[7]),
  ]
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
