pub mod foxel;

use getset::{CopyGetters, Getters, MutGetters};
use ultraviolet::Vec4;

use crate::{
  math::{hexadecitree::Hexadecitree, BlockPos},
  player::Player,
};

use self::foxel::Foxel;

pub struct World {
  pub foxels: Hexadecitree,
  pub sun_dir: Vec4,
}

impl World {
  pub fn new(sun_dir: Vec4) -> World {
    let foxels = Hexadecitree::new();
    Self { foxels, sun_dir }
  }

  pub fn setup_sample_scene(&mut self) {
    let f = &mut self.foxels;
    f.set(BlockPos::new(0, 0, 0, 0), Foxel::White).unwrap();
    for v in 1..10 {
      f.set(BlockPos::new(v, 0, 0, 0), Foxel::Red).unwrap();
      f.set(BlockPos::new(0, v, 0, 0), Foxel::Green).unwrap();
      f.set(BlockPos::new(0, 0, v, 0), Foxel::Blue).unwrap();
      f.set(BlockPos::new(0, 0, 0, v), Foxel::RB).unwrap();
    }
  }
}
