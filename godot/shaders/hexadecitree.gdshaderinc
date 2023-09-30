#include "global_uniforms.gdshaderinc"

const uint _H_HIGH_BIT = 1u << 15u;

// https://stackoverflow.com/questions/14997165/fastest-way-to-get-a-positive-modulo-in-c-c
uint H_rem(int i, uint n) {
  int ni = int(n);
  return uint((i % ni + ni) % ni);
}

// Return the idx'th byte of the tree.
// https://github.com/godotengine/godot/issues/57841
// due to problems this is more difficult than it has to be.
uint _H_index(sampler2D tree, uint idx) {
  uint width = uint(textureSize(tree, 0).x);
  uint idxOver4 = idx / 4u;
  uint subByte = idx % 4u;

  ivec2 coords = ivec2(
    int(idxOver4 % width),
    int(idxOver4 / width)
  );
  float raw = texelFetch(tree, coords, 0).r;
  uint bytes = floatBitsToUint(raw);
  uint shift = 4u - subByte;
  return (bytes >> (shift * 8u)) & 0xffu;
}

uint _H_getBrickPtrRepr(sampler2D tree, uint idx) {
  uint bytesIdx = idx * 2u;
  uint hi = _H_index(tree, bytesIdx);
  uint lo = _H_index(tree, bytesIdx + 1u);
  // little endian can eat my ass
  return (lo << 8u) | hi;
  // return (hi << 8u) | lo;
}

void _H_decompose1(int v, out uint gridPos, out uint foxelPos) {
  foxelPos = H_rem(v, TREE_FOXELS_ACROSS_BRICK);

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
  uint g = 0u;
  uint f = 0u;
  
  _H_decompose1(blockPos.x, g, f);
  gridIdx |= g; gridIdx *= TREE_BRICKS_ACROSS_WORLD;
  foxelIdx |= f; foxelIdx *= TREE_FOXELS_ACROSS_BRICK;
  _H_decompose1(blockPos.y, g, f);
  gridIdx |= g; gridIdx *= TREE_BRICKS_ACROSS_WORLD;
  foxelIdx |= f; foxelIdx *= TREE_FOXELS_ACROSS_BRICK;
  _H_decompose1(blockPos.z, g, f);
  gridIdx |= g; gridIdx *= TREE_BRICKS_ACROSS_WORLD;
  foxelIdx |= f; foxelIdx *= TREE_FOXELS_ACROSS_BRICK;
  _H_decompose1(blockPos.w, g, f);
  gridIdx |= g;
  foxelIdx |= f;

  return true;
}

bool H_get(sampler2D tree, ivec4 blockPos, out uint foxel) {
  uint gridIdx, foxelIdx;
  if (!_H_decomposePos(blockPos, gridIdx, foxelIdx)) {
    return false;
  }

  uint brickPtr = _H_getBrickPtrRepr(tree, gridIdx);
  uint highBit = 1u << 15u;
  if ((brickPtr & highBit) != 0u) {
    // Composite
    uint compositeIdx = brickPtr & ~highBit;
    uint foxelOffset = compositeIdx * TREE_FOXELS_PER_BRICK + foxelIdx;
    foxel = _H_index(tree, TREE_BRICKS_BYTES + foxelOffset);
    // foxel = (compositeIdx == 0u ? 2u : 1u);
  } else {
    // Solid brick
    foxel = brickPtr & 0xffu;
    // foxel = 5u;
  }
  return foxel != 0u;
}
