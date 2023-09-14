use tesseractory::{math::ChunkPos, world::Chunk};

#[test]
fn chunk_pos_round_trip() {
  let range = 50;
  for x in -range..range {
    for y in -range..range {
      for z in -range..range {
        for w in -range..range {
          let chunk = ChunkPos::new(x, y, z, w);
          let min_block = chunk.min_block();
          let reconstructed_chunk = min_block.chunk();
          assert_eq!(chunk, reconstructed_chunk);
        }
      }
    }
  }
}

#[test]
fn chunk_idx_round_trip() {
  for n in 0..Chunk::FOXEL_UCOUNT {
    let offset = Chunk::idx2offset(n);
    let reconstructed_n = Chunk::offset2idx(offset);
    assert_eq!(n, reconstructed_n);
  }
}
