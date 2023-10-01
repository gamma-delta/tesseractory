use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;

/// Foxes are imaginary creatures that exist only in dreams.
/// For reasons they can't explain, everyone knows what a fox looks like,
/// but no one can ever remember having seen one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
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

  Invalid = 255,
}

impl Foxel {
  pub fn transparent(&self) -> bool {
    match self {
      Foxel::Air => true,
      _ => false,
    }
  }

  pub fn encode(self) -> FoxelRepr {
    FoxelRepr(self as u8)
  }
}

/// Wrapper around sizeof Foxel, for easy shipping to the geepoo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(transparent)]
pub struct FoxelRepr(u8);

impl FoxelRepr {
  pub fn decode(self) -> Foxel {
    Foxel::try_from(self.0).unwrap_or(Foxel::Invalid)
  }
}
