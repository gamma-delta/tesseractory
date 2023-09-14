use godot::{
  engine::{image, Image, ImageTexture, Node},
  prelude::*,
};

use crate::GameState;

struct TesseractoryExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TesseractoryExtension {}

#[derive(GodotClass)]
#[class(base = Node)]
#[allow(dead_code)]
struct TesseractoryWorldHandler {
  world: GameState,

  canvas: Gd<Image>,
  #[var]
  canvas_tex: Gd<ImageTexture>,
  #[base]
  base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for TesseractoryWorldHandler {
  fn init(base: Base<Node>) -> Self {
    let canvas =
      Image::create(640, 480, false, image::Format::FORMAT_RGBA8).unwrap();
    let canvas_tex = ImageTexture::create_from_image(canvas.share()).unwrap();

    let camera =
      crate::Camera::new(canvas.get_width() as _, canvas.get_height() as _);
    let world = GameState::new(camera);

    Self {
      base,
      world,
      canvas,
      canvas_tex,
    }
  }
}

// This block is required for `#[var]` to work, for "technical reasons"
#[godot_api]
impl TesseractoryWorldHandler {}

impl TesseractoryWorldHandler {
  fn draw_world(&self) {}
}
