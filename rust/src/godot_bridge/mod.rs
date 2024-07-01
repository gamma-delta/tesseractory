mod math;

use godot::{
  engine::{image, Image, ImageTexture, RenderingServer, ShaderMaterial},
  prelude::*,
};

use crate::{
  godot_bridge::math::GdRotor4, math::hexadecitree::Hexadecitree, GameParams,
  TesseractoryGame,
};

/// https://github.com/godotengine/godot/issues/57841
const TREE_IMG_FORMAT: image::Format = image::Format::RF;

const VIEWPORT_WIDTH: u32 = 1000;
const VIEWPORT_HEIGHT: u32 = 600;

struct TesseractoryExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TesseractoryExtension {}

#[derive(GodotClass)]
#[class(base = Node)]
#[allow(dead_code)]
struct TesseractoryGodotBridge {
  #[export]
  cfg: Option<Gd<Resource>>,

  on_ready: Option<OnReadyStuff>,

  base: Base<Node>,
}

struct OnReadyStuff {
  /// Only exists on _ready
  game: TesseractoryGame,

  tree_image: Gd<Image>,
  tree_image_scratch: Vec<u8>,
  tree_tex: Gd<ImageTexture>,
}

#[godot_api]
impl INode for TesseractoryGodotBridge {
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
    let game = TesseractoryGame::new(params);

    let mut tree_image = Image::create(
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
    )
    .unwrap();
    let mut scratch = vec![0; Hexadecitree::TRANSFER_IMAGE_SIZE_SQ];
    let mut tree_tex =
      ImageTexture::create_from_image(tree_image.clone()).unwrap();

    game.world.foxels.upload(&mut scratch);
    tree_image.set_data(
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
      PackedByteArray::from(scratch.as_slice()),
    );
    tree_tex.update(tree_image.clone());

    self.on_ready = Some(OnReadyStuff {
      game,
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
}

#[godot_api]
impl TesseractoryGodotBridge {
  #[func]
  pub fn debug_string(&self) -> GString {
    let stuff = self.stuff();
    stuff.game.debug_info().into()
  }

  #[func]
  pub fn apply_per_tick_uniforms(&self, mut shader: Gd<ShaderMaterial>) {
    let stuff = self.stuff();

    let g_playerpos = vec4_to_gd(stuff.game.camera_pos);
    shader.set_shader_parameter("playerPos".into(), g_playerpos.to_variant());
    let look = stuff.game.camera_rot;
    let uughgh = array![
      look.s, look.bv.xy, look.bv.xz, look.bv.xw, look.bv.yz, look.bv.yw,
      look.bv.zw, look.p,
    ];
    shader.set_shader_parameter("playerLookRaw".into(), uughgh.to_variant());

    let cfg = &stuff.game.params;
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
    self.stuff().tree_tex.clone()
  }

  #[func]
  pub fn viewport_size(&self) -> Vector2i {
    Vector2i::new(VIEWPORT_WIDTH as _, VIEWPORT_HEIGHT as _)
  }

  // Player API stuff

  #[func]
  pub fn render_from(&mut self, pos: Vector4, rot: Gd<GdRotor4>) {
    let stuff = self.stuff_mut();
    stuff.game.camera_pos = vec4_from_gd(pos);
    stuff.game.camera_rot = rot.bind().inner;
  }
}

impl TesseractoryGodotBridge {
  fn stuff(&self) -> &OnReadyStuff {
    self.on_ready.as_ref().unwrap()
  }

  fn stuff_mut(&mut self) -> &mut OnReadyStuff {
    self.on_ready.as_mut().unwrap()
  }
}

// Helper fns

pub fn vec4_from_gd(v: Vector4) -> ultraviolet::Vec4 {
  ultraviolet::Vec4::new(v.x, v.y, v.z, v.w)
}

pub fn vec4_to_gd(v: ultraviolet::Vec4) -> Vector4 {
  Vector4::new(v.x, v.y, v.z, v.w)
}
