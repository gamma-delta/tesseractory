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
  pub fn global_shader_uniforms(&self) -> PackedByteArray {
    // https://stackoverflow.com/questions/60469505/how-can-i-create-a-tightly-packed-uniform-buffer-of-unsigned-bytes
    // Apparently the gpu uses the same endianness as the cpu.
    let mut p = PackedByteArray::new();

    p.extend((Hexadecitree::DEPTH as u32).to_ne_bytes());
    p.extend((Hexadecitree::MAX_COORD as u32).to_ne_bytes());
    p.extend((Hexadecitree::MIN_COORD as u32).to_ne_bytes());
    p
  }

  #[func]
  pub fn hexadecitree_uniform(&self) -> PackedByteArray {
    let mut p = PackedByteArray::new();

    let branch_pad = 65536;
    let foxel_pad = 8192; // note this is in u32s, not bytes

    let tree = self.world_state.as_ref().unwrap().world.foxels();
    let branches = tree.branches_to_gpu();
    let foxels = tree.foxels_to_gpu();

    if branches.len() > branch_pad {
      panic!(
        "tried to submit {} branches to the gpu but only have space for {}",
        branches.len(),
        branch_pad
      );
    }
    p.extend(bytemuck::bytes_of(branches).into_iter().copied());
    p.extend(std::iter::repeat(0).take((branch_pad - branches.len()) / 4));

    if foxels.len() > foxel_pad {
      panic!(
        "tried to submit {} foxels to the gpu but only have space for {}",
        foxels.len(),
        foxel_pad
      );
    }
    p.extend(bytemuck::bytes_of(foxels).into_iter().copied());
    p.extend(std::iter::repeat(0).take((foxel_pad - foxels.len()) / 4));

    p
  }

  #[func]
  pub fn debug_string(&self) -> GodotString {
    self.world_state.as_ref().unwrap().debug_info().into()
  }
}
