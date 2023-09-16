use itertools::iproduct;
use tesseractory::{
  math::BlockPos,
  world::{Foxel, Hexadecitree},
};

#[test]
fn smoke() {
  let mut h = Hexadecitree::new();
  let len = 10;
  for (x, y, z, w) in iproduct!(-len..len, -len..len, -len..len, -len..len) {
    let foxel = Foxel::ColorBlock(x & 1 == 0, y & 1 == 0, z & 1 == 0);
    let pos = BlockPos::new(x, y, z, w);
    h.set(pos, foxel);
    let retrieve = h.get(pos);
    assert_eq!(retrieve, Some(foxel));
  }
}
