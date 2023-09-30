use godot::{
  engine::{image, Image, ImageTexture, RenderingServer, ShaderMaterial},
  prelude::*,
};

use crate::{math::hexadecitree::Hexadecitree, GameParams, WorldState};

const TREE_IMG_FORMAT: image::Format = image::Format::FORMAT_R8;

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

  tree_image: Gd<Image>,
  tree_image_scratch: Vec<u8>,
  #[var]
  tree_tex: Gd<ImageTexture>,

  #[base]
  base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for TesseractoryWorldHandler {
  fn init(base: Base<Node>) -> Self {
    // apparently is initialized elsewhere
    let _ = env_logger::try_init();

    let image = Image::create(
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
    )
    .unwrap();
    let scratch = vec![0; Hexadecitree::TRANSFER_IMAGE_SIZE_SQ];
    let tex = ImageTexture::create_from_image(image.share()).unwrap();

    Self {
      base,

      world_state: None,

      tree_image: image,
      tree_image_scratch: scratch,
      tree_tex: tex,

      cfg: None,
    }
  }

  fn ready(&mut self) {
    let params = GameParams::load(self.cfg.as_ref().unwrap());
    self.world_state = Some(WorldState::new(params));

    let world = self.world_state.as_ref().unwrap();
    world.world.foxels().upload(&mut self.tree_image_scratch);
    self.tree_image.set_data(
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
      PackedByteArray::from(self.tree_image_scratch.as_slice()),
    );
    self.tree_tex.update(self.tree_image.share());

    let mut rs = RenderingServer::singleton();
    for (k, v) in [
      (
        "TREE_COMPOSITE_BRICK_COUNT",
        (Hexadecitree::COMPOSITE_BRICK_COUNT as u32).to_variant(),
      ),
      (
        "TREE_FOXELS_ACROSS_BRICK",
        (Hexadecitree::FOXELS_ACROSS_BRICK as u32).to_variant(),
      ),
      (
        "TREE_BRICKS_ACROSS_WORLD",
        (Hexadecitree::BRICKS_ACROSS_WORLD as u32).to_variant(),
      ),
      (
        "TREE_BRICKS_BYTES",
        (Hexadecitree::BRICKS_BYTES as u32).to_variant(),
      ),
      ("TREE_MIN_COORD", Hexadecitree::MIN_COORD.to_variant()),
      ("TREE_MAX_COORD", Hexadecitree::MAX_COORD.to_variant()),
    ] {
      rs.global_shader_parameter_set(k.into(), v);
    }
  }

  fn process(&mut self, delta: f64) {}

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

  #[func]
  pub fn apply_per_tick_uniforms(&self, mut shader: Gd<ShaderMaterial>) {
    let w = self.world_state.as_ref().unwrap();

    let pp = w.player.pos();
    let g_playerpos = Vector4::new(pp.x, pp.y, pp.z, pp.w);
    shader.set_shader_parameter("playerPos".into(), g_playerpos.to_variant());
  }
}
