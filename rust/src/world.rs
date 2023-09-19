use getset::{CopyGetters, Getters};
use godot::prelude::Color;
use ultraviolet::Vec4;

use crate::math::{hexadecitree::Hexadecitree, BlockPos};

#[derive(CopyGetters, Getters)]
pub struct World {
  #[getset(get = "pub")]
  foxels: Hexadecitree,
  #[getset(get_copy = "pub")]
  sun_dir: Vec4,
}

impl World {
  pub fn new(sun_dir: Vec4) -> World {
    let foxels = Hexadecitree::new();
    Self { foxels, sun_dir }
  }

  pub fn setup_sample_scene(&mut self) {
    let f = &mut self.foxels;
    f.set(
      BlockPos::new(0, 0, 0, 0),
      Foxel::ColorBlock(true, true, true),
    );
    for v in 1..10 {
      f.set(
        BlockPos::new(v, 0, 0, 0),
        Foxel::ColorBlock(true, false, false),
      );
      f.set(
        BlockPos::new(0, v, 0, 0),
        Foxel::ColorBlock(false, true, false),
      );
      f.set(
        BlockPos::new(0, 0, v, 0),
        Foxel::ColorBlock(false, false, true),
      );
      f.set(
        BlockPos::new(0, 0, 0, v),
        Foxel::ColorBlock(true, false, true),
      );
    }
  }

  pub fn get_foxel(&self, pos: BlockPos) -> Option<Foxel> {
    self.foxels.get(pos)
  }

  pub fn get_foxel_mut(&mut self, pos: BlockPos) -> Option<&mut Foxel> {
    self.foxels.get_mut(pos)
  }

  pub fn set_foxel(&mut self, pos: BlockPos, foxel: Foxel) -> Option<Foxel> {
    self.foxels.set(pos, foxel)
  }
}

/// Foxes are imaginary creatures that exist only in dreams.
/// For reasons they can't explain, everyone knows what a fox looks like,
/// but no one can ever remember having seen one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Foxel {
  Air,
  ColorBlock(bool, bool, bool),
}

impl Foxel {
  pub fn transparent(&self) -> bool {
    match self {
      Foxel::Air => true,
      _ => false,
    }
  }

  pub fn color(&self) -> Color {
    match self {
      Foxel::Air => panic!(),
      &Foxel::ColorBlock(r, g, b) => {
        let r = r as u8 * 255;
        let g = g as u8 * 255;
        let b = b as u8 * 255;
        Color::from_rgba8(r, g, b, 255)
      }
    }
  }
}
