use itertools::iproduct;
use tesseractory::{
  math::{hexadecitree::Hexadecitree, BlockPos},
  world::Foxel,
};

#[test]
fn smoke() {
  let palette = [
    Foxel::Red,
    Foxel::Green,
    Foxel::Blue,
    Foxel::RG,
    Foxel::GB,
    Foxel::RB,
    Foxel::White,
    Foxel::Black,
  ];

  let mut h = Hexadecitree::new();
  let len = 10i32;
  for (x, y, z, w) in iproduct!(-len..len, -len..len, -len..len, -len..len) {
    let foxel =
      palette[(x + y + z + w).rem_euclid(palette.len() as i32) as usize];
    let pos = BlockPos::new(x, y, z, w);
    h.set(pos, foxel).unwrap();
    let retrieve = h.get(pos);
    assert_eq!(retrieve, Some(foxel), "{:?}", pos);
  }

  // panic!("{}", h.memory());
}
