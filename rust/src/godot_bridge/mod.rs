mod math;

pub use math::GdPlayerCamera;

use std::time::Instant;

use godot::{
  engine::{image, Image, ImageTexture, RenderingServer},
  prelude::*,
};

use crate::{math::hexadecitree::Hexadecitree, TesseractoryGame};

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
  on_ready: Option<OnReadyStuff>,

  base: Base<Node>,
}

struct OnReadyStuff {
  /// Only exists on _ready
  game: TesseractoryGame,

  /// Keep this around to avoid having to realloc all the time
  tree_scratch: PackedByteArray,
  tree_image: Gd<Image>,
  tree_tex: Gd<ImageTexture>,
}

#[godot_api]
impl INode for TesseractoryGodotBridge {
  fn init(base: Base<Node>) -> Self {
    // apparently is initialized elsewhere
    let _ = env_logger::try_init();

    Self {
      base,
      on_ready: None,
    }
  }

  fn ready(&mut self) {
    let game = TesseractoryGame::new();

    let scratch = PackedByteArray::from(
      vec![0u8; Hexadecitree::GPU_TRANSFER_IMAGE_SIZE_SQ * 4].as_slice(),
    );
    let tree_image = Image::create_from_data(
      Hexadecitree::GPU_TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::GPU_TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
      scratch.clone(),
    )
    .unwrap();
    let tree_tex = ImageTexture::create_from_image(tree_image.clone()).unwrap();
    self.on_ready = Some(OnReadyStuff {
      game,
      tree_tex,
      tree_image,
      tree_scratch: scratch,
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
        (Hexadecitree::GPU_BRICK_PTRS_BYTES as u32).to_variant(),
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
  pub fn tree_tex(&self) -> Gd<ImageTexture> {
    self.stuff().tree_tex.clone()
  }

  #[func]
  pub fn viewport_size(&self) -> Vector2i {
    Vector2i::new(VIEWPORT_WIDTH as _, VIEWPORT_HEIGHT as _)
  }

  #[func]
  pub fn upload_foxels(&mut self, cam: Gd<GdPlayerCamera>) {
    let now = Instant::now();

    let stuff = self.stuff_mut();
    stuff
      .game
      .world
      .foxels
      .upload(stuff.tree_scratch.as_mut_slice(), &*cam.bind());
    stuff.tree_image.set_data(
      Hexadecitree::GPU_TRANSFER_IMAGE_SIZE as i32,
      Hexadecitree::GPU_TRANSFER_IMAGE_SIZE as i32,
      false,
      TREE_IMG_FORMAT,
      stuff.tree_scratch.clone(),
    );
    stuff.tree_tex.update(stuff.tree_image.clone());

    let time = Instant::now() - now;
    godot_print!(
      "upload fps: {}; img size {}",
      1.0 / time.as_secs_f32(),
      Hexadecitree::GPU_TRANSFER_IMAGE_SIZE
    );
  }
}

impl TesseractoryGodotBridge {
  #[inline]
  fn stuff(&self) -> &OnReadyStuff {
    self.on_ready.as_ref().unwrap()
  }

  #[inline]
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
