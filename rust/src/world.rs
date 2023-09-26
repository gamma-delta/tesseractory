use bytemuck::NoUninit;
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
    f.set(BlockPos::new(0, 0, 0, 0), Foxel::White);
    for v in 1..10 {
      f.set(BlockPos::new(v, 0, 0, 0), Foxel::Red);
      f.set(BlockPos::new(0, v, 0, 0), Foxel::Green);
      f.set(BlockPos::new(0, 0, v, 0), Foxel::Blue);
      f.set(BlockPos::new(0, 0, 0, v), Foxel::RB);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, NoUninit)]
#[repr(u8)]
pub enum Foxel {
  Air,
  Red,
  Green,
  Blue,
  RG,
  GB,
  RB,
  Black,
  White,
}

impl Foxel {
  pub fn transparent(&self) -> bool {
    match self {
      Foxel::Air => true,
      _ => false,
    }
  }

  pub fn color(&self) -> Color {
    let t = 1.0;
    let f = 0.0;

    let [r, g, b] = match self {
      Foxel::Air => panic!(),
      Foxel::Red => [t, f, f],
      Foxel::Green => [f, t, f],
      Foxel::Blue => [f, f, t],
      Foxel::RG => [t, t, f],
      Foxel::GB => [f, t, t],
      Foxel::RB => [t, f, t],
      Foxel::Black => [f, f, f],
      Foxel::White => [t, t, t],
    };

    Color::from_rgba(r, g, b, 1.0)
  }
}
