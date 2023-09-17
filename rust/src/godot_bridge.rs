use godot::{
  engine::{image, Image, ImageTexture, Node},
  prelude::*,
};
use ultraviolet::IVec2;

const CANVAS_FORMAT: image::Format = image::Format::FORMAT_RGB8;

use crate::{GameParams, GameState};

struct TesseractoryExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TesseractoryExtension {}

#[derive(GodotClass)]
#[class(base = Node)]
#[allow(dead_code)]
struct TesseractoryWorldHandler {
  /// Only exists on _ready
  game: Option<GameState>,

  canvas: Gd<Image>,
  canvas_tex: Gd<ImageTexture>,
  canvas_scratch: Vec<[u8; 3]>,
  canvas_scratch_pba: PackedByteArray,

  #[export]
  cfg: Option<Gd<Resource>>,

  #[base]
  base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for TesseractoryWorldHandler {
  fn init(base: Base<Node>) -> Self {
    let canvas = Image::create(640, 480, false, CANVAS_FORMAT).unwrap();
    let mut canvas_tex = ImageTexture::new();
    canvas_tex.set_image(canvas.share());

    Self {
      base,

      game: None,
      canvas,
      canvas_tex,
      canvas_scratch: vec![[0, 0, 0]; 640 * 480],
      canvas_scratch_pba: PackedByteArray::new(),

      cfg: None,
    }
  }

  fn ready(&mut self) {
    let params = GameParams::load(self.cfg.as_ref().unwrap());
    self.game = Some(GameState::new(
      IVec2::new(self.canvas.get_width(), self.canvas.get_height()),
      params,
    ));
  }

  fn physics_process(&mut self, delta: f64) {
    self.game.as_mut().unwrap().physics_process(delta as f32);
  }

  fn process(&mut self, delta: f64) {
    self
      .game
      .as_mut()
      .unwrap()
      .draw_world(&mut self.canvas_scratch);
    let scratch_bytes = bytemuck::cast_slice(self.canvas_scratch.as_slice());
    self.canvas_scratch_pba.resize(scratch_bytes.len());
    self
      .canvas_scratch_pba
      .as_mut_slice()
      .copy_from_slice(scratch_bytes);
    self.canvas.set_data(
      640,
      480,
      false,
      CANVAS_FORMAT,
      // COW, so cloning is ok
      self.canvas_scratch_pba.clone(),
    );
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
    self.game.as_ref().unwrap().debug_info().into()
  }
}

#[derive(Debug, Clone, Copy)]
pub struct GodotEditorConfig {
  pub fov: f32,
  pub focal_dist: f32,
}
