pub mod extensions;
pub mod godot_bridge;
pub mod math;
pub mod player;
pub mod world;

use extensions::GodotObjectExt;
use godot::prelude::{Color, Gd, Resource};
use math::{geo::Rotor4, hexadecitree::Hexadecitree};
use ultraviolet::Vec4;
use world::{foxel::Foxel, World};

pub struct TesseractoryGame {
  world: World,
  camera_pos: Vec4,
  camera_rot: Rotor4,

  params: GameParams,
}

impl TesseractoryGame {
  pub fn new(params: GameParams) -> Self {
    let sun_dir = Vec4::new(-0.5, 0.4, 0.2, 0.1).normalized();
    let mut world = World::new(sun_dir);
    world.setup_sample_scene();

    Self {
      world,
      camera_pos: Vec4::zero(),
      camera_rot: Rotor4::identity(),
      params,
    }
  }

  pub fn debug_info(&self) -> String {
    let mut w = String::new();

    w += &format!(
      "FPS: {}\n",
      godot::engine::Engine::singleton().get_frames_per_second()
    );

    w += &format!(
      "Composite bricks: {} / {}\n",
      self.world.foxels.composite_brick_count(),
      Hexadecitree::COMPOSITE_BRICK_COUNT,
    );

    w
  }
}

pub struct GameParams {
  pub focal_dist: f32,
  pub fov: f32,
}

impl GameParams {
  pub fn load(cfg: &Gd<Resource>) -> Self {
    Self {
      focal_dist: cfg.totally::<f32>("focal_dist") / 1_000.0,
      fov: cfg.totally::<f32>("fov"),
    }
  }
}
