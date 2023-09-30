#include "global_uniforms.gdshaderinc"

const uint _H_HIGH_BIT = 1u << 15u;

// https://stackoverflow.com/questions/14997165/fastest-way-to-get-a-positive-modulo-in-c-c
uint H_rem(int i, int n) {
  return uint((i % n + n) % n);
}

// Return the idx'th byte of the tree.
uint _H_index(usampler2D tree, uint idx) {
  uint w = uint(textureSize(tree, 0).x);

  ivec2 coords = ivec2(int(idx % w), int(idx / w));
  return texelFetch(tree, coords, 0).r;
}

uint _H_getBrickPtrRepr(usampler2D tree, uint idx) {
  uint bytesIdx = idx * 2u;
  uint hi = _H_index(tree, bytesIdx);
  uint lo = _H_index(tree, bytesIdx + 1u);
  return (hi << 8u) | lo;
}

void _H_decompose1(int v, out uint gridPos, out uint foxelPos) {
  foxelPos = H_rem(v, int(TREE_FOXELS_ACROSS_BRICK));
  int rawGridPos = (v >= 0)
    ? v / int(TREE_FOXELS_ACROSS_BRICK)
    : v / int(TREE_FOXELS_ACROSS_BRICK) - 1;
  gridPos = uint(rawGridPos + int(TREE_BRICKS_ACROSS_WORLD) / 2);
}

// Return the index of the brick it's in, then (if the brick isn't solid)
// the index of the position in the brick
bool _H_decomposePos(
  ivec4 blockPos, out uint gridIdx, out uint foxelIdx
) {
  if (any(lessThan(blockPos, ivec4(TREE_MIN_COORD)))
    || any(greaterThan(blockPos, ivec4(TREE_MAX_COORD)))) {
    return false;
  }

  gridIdx = 0u;
  foxelIdx = 0u;
  // scratch
  uint g, f;
  
  _H_decompose1(blockPos.x, g, f);
  gridIdx |= g; gridIdx *= TREE_FOXELS_ACROSS_BRICK;
  foxelIdx |= f; foxelIdx *= TREE_BRICKS_ACROSS_WORLD;
  _H_decompose1(blockPos.y, g, f);
  gridIdx |= g; gridIdx *= TREE_FOXELS_ACROSS_BRICK;
  foxelIdx |= f; foxelIdx *= TREE_BRICKS_ACROSS_WORLD;
  _H_decompose1(blockPos.z, g, f);
  gridIdx |= g; gridIdx *= TREE_FOXELS_ACROSS_BRICK;
  foxelIdx |= f; foxelIdx *= TREE_BRICKS_ACROSS_WORLD;
  _H_decompose1(blockPos.w, g, f);
  gridIdx |= g;
  foxelIdx |= f;

  return true;
}

bool H_get(usampler2D tree, ivec4 blockPos, out uint foxel) {
  uint gridIdx, foxelIdx;
  if (!_H_decomposePos(blockPos, gridIdx, foxelIdx)) {
    return false;
  }

  uint brickPtr = _H_getBrickPtrRepr(tree, gridIdx);
  uint highBit = 1u << 15u;
  if ((brickPtr & highBit) != 0u) {
    // Composite
    uint brickIdx = brickPtr & ~highBit;
    foxel = _H_index(tree, TREE_BRICKS_BYTES 
      + TREE_FOXELS_ACROSS_BRICK * brickIdx * 2u
      + foxelIdx);
  } else {
    // Solid brick
    foxel = brickPtr & 0xffu;
  }
  return foxel != 0u;
}
