pub mod extensions;
pub mod godot_bridge;
pub mod math;
pub mod player;
pub mod world;

use extensions::GodotObjectExt;
use godot::prelude::{Color, Gd, Resource};
use math::hexadecitree::Hexadecitree;
use player::Player;
use ultraviolet::Vec4;
use world::{foxel::Foxel, World};

pub struct WorldState {
  world: World,
  player: Player,

  params: GameParams,
}

impl WorldState {
  pub fn new(params: GameParams) -> Self {
    let sun_dir = Vec4::new(-0.5, 0.4, 0.2, 0.1).normalized();
    let mut world = World::new(sun_dir);
    world.setup_sample_scene();

    let player = Player::new(Vec4::new(0.0, -3.0, 0.001, 0.5));
    // let player = Player::new(Vec4::new(0.0, 0.0, 0.0, 0.0));

    Self {
      world,
      player,
      params,
    }
  }

  pub fn physics_process(&mut self, delta: f32) {
    self.player.physics_process(delta, &self.params);
  }

  pub fn debug_info(&self) -> String {
    let mut w = String::new();

    w += &format!(
      "FPS: {}\n",
      godot::engine::Engine::singleton().get_frames_per_second()
    );

    self.player.debug_info(&mut w);

    w += &format!(
      "Composite bricks: {} / {}\n",
      self.world.foxels().composite_brick_count(),
      Hexadecitree::COMPOSITE_BRICK_COUNT,
    );

    w
  }
}

pub struct GameParams {
  pub focal_dist: f32,
  pub fov: f32,
  pub player_walk_speed: f32,
  pub player_fly_speed: f32,
  pub look_speed: f32,
}

impl GameParams {
  pub fn load(cfg: &Gd<Resource>) -> Self {
    Self {
      focal_dist: cfg.totally::<f32>("focal_dist") / 1_000.0,
      fov: cfg.totally::<f32>("fov") / 10_000.0,
      player_walk_speed: cfg.totally("walk_speed"),
      player_fly_speed: cfg.totally("fly_speed"),
      look_speed: cfg.totally::<f32>("look_speed") / 100.0,
    }
  }
}
