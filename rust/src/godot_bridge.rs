use godot::{
  engine::{image, Image, ImageTexture, RenderingServer, ShaderMaterial},
  prelude::*,
};

use crate::{math::hexadecitree::Hexadecitree, GameParams, WorldState};

/// https://github.com/godotengine/godot/issues/57841
const TREE_IMG_FORMAT: image::Format = image::Format::FORMAT_RF;

const VIEWPORT_WIDTH: u32 = 640;
const VIEWPORT_HEIGHT: u32 = 480;

struct TesseractoryExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TesseractoryExtension {}

#[derive(GodotClass)]
#[class(base = Node)]
#[allow(dead_code)]
struct TesseractoryWorldHandler {
  #[export]
  cfg: Option<Gd<Resource>>,

  on_ready: Option<OnReadyStuff>,

  #[base]
  base: Base<Node>,
}

struct OnReadyStuff {
  /// Only exists on _ready
  world_state: WorldState,

  tree_image: Gd<Image>,
  tree_image_scratch: Vec<u8>,
  tree_tex: Gd<ImageTexture>,
}

#[godot_api]
impl NodeVirtual for TesseractoryWorldHandler {
  fn init(base: Base<Node>) -> Self {
    // apparently is initialized elsewhere
    let _ = env_logger::try_init();

    Self {
      base,
      cfg: None,
      on_ready: None,
    }
  }

  fn ready(&mut self) {
    let params = GameParams::load(self.cfg.as_ref().unwrap());
    let world_state = WorldState::new(params);

    let mut tree_image = Image::create(
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
    )
    .unwrap();
    let mut scratch = vec![0; Hexadecitree::TRANSFER_IMAGE_SIZE_SQ];
    let mut tree_tex =
      ImageTexture::create_from_image(tree_image.share()).unwrap();

    world_state.world.foxels().upload(&mut scratch);
    tree_image.set_data(
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
      PackedByteArray::from(scratch.as_slice()),
    );
    tree_tex.update(tree_image.share());

    self.on_ready = Some(OnReadyStuff {
      world_state,
      tree_image,
      tree_image_scratch: scratch,
      tree_tex,
    });

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
        "TREE_FOXELS_PER_BRICK",
        (Hexadecitree::FOXELS_PER_BRICK as u32).to_variant(),
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
    self.stuff_mut().world_state.physics_process(delta as f32);
  }
}

#[godot_api]
impl TesseractoryWorldHandler {
  #[func]
  pub fn debug_string(&self) -> GodotString {
    let stuff = self.stuff();
    stuff.world_state.debug_info().into()
  }

  #[func]
  pub fn apply_per_tick_uniforms(&self, mut shader: Gd<ShaderMaterial>) {
    let stuff = self.stuff();

    let pp = stuff.world_state.player.pos();
    let g_playerpos = Vector4::new(pp.x, pp.y, pp.z, pp.w);
    shader.set_shader_parameter("playerPos".into(), g_playerpos.to_variant());
    let look = stuff.world_state.player.look();
    let uughgh = array![
      look.s, look.bv.xy, look.bv.xz, look.bv.xw, look.bv.yz, look.bv.yw,
      look.bv.zw, look.p,
    ];
    shader.set_shader_parameter("playerLookRaw".into(), uughgh.to_variant());

    let cfg = &stuff.world_state.params;
    shader
      .set_shader_parameter("focalDist".into(), cfg.focal_dist.to_variant());
    shader.set_shader_parameter("fov".into(), cfg.fov.to_variant());

    shader.set_shader_parameter(
      "aspectRatio".into(),
      (VIEWPORT_WIDTH as f32 / VIEWPORT_HEIGHT as f32).to_variant(),
    );
  }

  #[func]
  pub fn tree_tex(&self) -> Gd<ImageTexture> {
    self.stuff().tree_tex.share()
  }

  #[func]
  pub fn viewport_size(&self) -> Vector2i {
    Vector2i::new(VIEWPORT_WIDTH as _, VIEWPORT_HEIGHT as _)
  }
}

impl TesseractoryWorldHandler {
  fn stuff(&self) -> &OnReadyStuff {
    self.on_ready.as_ref().unwrap()
  }

  fn stuff_mut(&mut self) -> &mut OnReadyStuff {
    self.on_ready.as_mut().unwrap()
  }
}
