use glam::IVec2;
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
  game: GameState,

  canvas: Gd<Image>,
  canvas_tex: Gd<ImageTexture>,
  #[base]
  base: Base<Node>,

  #[export(range = (0.05, 5.0))]
  fov: f32,
  #[export(range = (0.05, 5.0))]
  focal_dist: f32,
}

#[godot_api]
impl NodeVirtual for TesseractoryWorldHandler {
  fn init(base: Base<Node>) -> Self {
    let canvas =
      Image::create(640, 480, false, image::Format::FORMAT_RGB8).unwrap();
    let canvas_tex = ImageTexture::create_from_image(canvas.share()).unwrap();

    let camera =
      crate::Camera::new(canvas.get_width() as _, canvas.get_height() as _);
    let game = GameState::new(camera);

    Self {
      base,

      game,
      canvas,
      canvas_tex,

      fov: 0.1,
      focal_dist: 0.7,
    }
  }

  fn process(&mut self, delta: f64) {
    let cfg = GodotEditorConfig {
      fov: self.fov / 10_000.0,
      focal_dist: self.focal_dist / 100.0,
    };
    self.game.update(cfg, delta as f32);

    let canvas_wrapper = CanvasWrapper {
      image: &mut self.canvas,
    };
    self.game.draw_world(canvas_wrapper);

    self.canvas_tex.update(self.canvas.share());
  }
}

// This block is required for `#[var]` to work, for "technical reasons"
#[godot_api]
impl TesseractoryWorldHandler {
  #[func]
  pub fn get_canvas_tex(&self) -> Gd<ImageTexture> {
    self.canvas_tex.share()
  }

  #[func]
  pub fn debug_string(&self) -> GodotString {
    self.game.debug_info().into()
  }
}

pub struct CanvasWrapper<'a> {
  image: &'a mut Image,
}

impl<'a> CanvasWrapper<'a> {
  pub fn set_pixel(&mut self, pos: IVec2, color: Color) {
    self.image.set_pixel(pos.x, pos.y, color);
  }
}

#[derive(Debug, Clone, Copy)]
pub struct GodotEditorConfig {
  pub fov: f32,
  pub focal_dist: f32,
}
