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
  canvas_scratch: Vec<[f32; 3]>,
  canvas_scratch_pba: PackedByteArray,
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
      Image::create(640, 480, false, image::Format::FORMAT_RGBF).unwrap();
    let mut canvas_tex = ImageTexture::new();
    canvas_tex.set_image(canvas.share());

    let camera =
      crate::Camera::new(canvas.get_width() as _, canvas.get_height() as _);
    let game = GameState::new(camera);

    Self {
      base,

      game,
      canvas,
      canvas_tex,
      canvas_scratch: vec![[0.0, 0.0, 0.0]; 640 * 480],
      canvas_scratch_pba: PackedByteArray::new(),

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

    self.game.draw_world(&mut self.canvas_scratch);
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
      image::Format::FORMAT_RGBF,
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
    self.game.debug_info().into()
  }
}

#[derive(Debug, Clone, Copy)]
pub struct GodotEditorConfig {
  pub fov: f32,
  pub focal_dist: f32,
}
