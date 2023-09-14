use ahash::AHashMap;
use glam::IVec4;

use crate::math::{BlockPos, ChunkPos};

pub struct FoxelStore {
  // evil
  chunks: AHashMap<ChunkPos, Chunk>,
}

impl FoxelStore {
  pub fn new() -> Self {
    Self {
      chunks: AHashMap::new(),
    }
  }

  pub fn sample_scene(&mut self) {
    let foxel = self.foxel_at_mut(BlockPos::new(3, 2, 0, 0)).unwrap();
    *foxel = FoxelType::RedBlock;
  }

  /// If there exists a chunk with the given coordinate, return it
  pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
    self.chunks.get(&pos)
  }

  /// Get the chunk at the given pos, creating it if it doesn't exist.
  pub fn expect_chunk_for(&mut self, pos: ChunkPos) -> &mut Chunk {
    self.chunks.entry(pos).or_insert_with(|| Chunk::new(pos))
  }

  pub fn foxel_at(&self, pos: BlockPos) -> Option<FoxelType> {
    let chunk_pos = pos.chunk();
    let chunk = self.chunks.get(&chunk_pos)?;
    chunk.get_foxel(pos)
  }

  pub fn foxel_at_mut(&mut self, pos: BlockPos) -> Option<&mut FoxelType> {
    let chunk_pos = pos.chunk();
    let chunk = self.chunks.get_mut(&chunk_pos)?;
    chunk.get_foxel_mut(pos)
  }
}

pub struct Chunk {
  pos: ChunkPos,
  foxels: [FoxelType; Chunk::FOXEL_COUNT as _],
}

impl Chunk {
  // a chunk size of 8 makes for 4096 foxels.
  // this is the same number as a 16-size voxel chunk.
  pub const BREADTH: i32 = 8;
  pub const UBREADTH: usize = Chunk::BREADTH as usize;
  pub const FOXEL_COUNT: i32 = Chunk::BREADTH.pow(4);
  pub const FOXEL_UCOUNT: usize = Chunk::FOXEL_COUNT as usize;

  pub fn new(pos: ChunkPos) -> Self {
    Self {
      pos,
      foxels: [FoxelType::Air; Chunk::FOXEL_COUNT as _],
    }
  }

  pub fn pos(&self) -> ChunkPos {
    self.pos
  }

  /// Returns None if the pos is outside of this chunk
  pub fn get_foxel(&self, pos: BlockPos) -> Option<FoxelType> {
    let offset = self.pos.contained_offset(pos)?;
    let idx = Chunk::offset2idx(offset);
    Some(self.foxels[idx])
  }

  pub fn get_foxel_mut(&mut self, pos: BlockPos) -> Option<&mut FoxelType> {
    let offset = self.pos.contained_offset(pos)?;
    let idx = Chunk::offset2idx(offset);
    Some(&mut self.foxels[idx])
  }

  // https://stackoverflow.com/questions/7367770/how-to-flatten-or-index-3d-array-in-1d-array
  pub fn offset2idx(offset: IVec4) -> usize {
    let x: usize = offset.x.try_into().unwrap();
    let y: usize = offset.y.try_into().unwrap();
    let z: usize = offset.z.try_into().unwrap();
    let w: usize = offset.w.try_into().unwrap();
    debug_assert!(x <= Chunk::UBREADTH);
    debug_assert!(y <= Chunk::UBREADTH);
    debug_assert!(z <= Chunk::UBREADTH);
    debug_assert!(w <= Chunk::UBREADTH);

    x + Chunk::UBREADTH * (y + Chunk::UBREADTH * (z + Chunk::UBREADTH * w))
  }

  pub fn idx2offset(idx: usize) -> IVec4 {
    let mut idx = idx;
    let x = idx % Chunk::UBREADTH;
    idx /= Chunk::UBREADTH;
    let y = idx % Chunk::UBREADTH;
    idx /= Chunk::UBREADTH;
    let z = idx % Chunk::UBREADTH;
    idx /= Chunk::UBREADTH;
    let w = idx % Chunk::UBREADTH;
    IVec4::new(x as _, y as _, z as _, w as _)
  }
}

/// Foxes are imaginary creatures that exist only in dreams.
/// For reasons they can't explain, everyone knows what a fox looks like,
/// but no one can ever remember having seen one.
#[derive(Debug, Clone, Copy)]
pub enum FoxelType {
  Air,
  RedBlock,
  GreenBlock,
  BlueBlock,
}
