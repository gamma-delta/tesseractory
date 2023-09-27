use godot::{engine::RenderingServer, prelude::*};

use crate::{math::hexadecitree::Hexadecitree, GameParams, WorldState};

struct TesseractoryExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TesseractoryExtension {}

#[derive(GodotClass)]
#[class(base = Node)]
#[allow(dead_code)]
struct TesseractoryWorldHandler {
  /// Only exists on _ready
  world_state: Option<WorldState>,

  #[export]
  cfg: Option<Gd<Resource>>,
  #[export]
  screen: Option<Gd<Node>>,

  #[base]
  base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for TesseractoryWorldHandler {
  fn init(base: Base<Node>) -> Self {
    Self {
      base,

      world_state: None,

      screen: None,
      cfg: None,
    }
  }

  fn ready(&mut self) {
    let params = GameParams::load(self.cfg.as_ref().unwrap());
    self.world_state = Some(WorldState::new(params));
  }

  fn physics_process(&mut self, delta: f64) {
    self
      .world_state
      .as_mut()
      .unwrap()
      .physics_process(delta as f32);
  }
}

#[godot_api]
impl TesseractoryWorldHandler {
  #[func]
  pub fn debug_string(&self) -> GodotString {
    self.world_state.as_ref().unwrap().debug_info().into()
  }
}
