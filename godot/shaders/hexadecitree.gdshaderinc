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
  // uint shift = 3u - subByte;
  uint shift = subByte;
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

#define H_GET_OOB 0u
#define H_GET_AIR_BRICK 1u
#define H_GET_FOXEL 2u

// Returns H_GET*
uint H_get(sampler2D tree, ivec4 blockPos, out uint foxel) {
  uint gridIdx, foxelIdx;
  if (!_H_decomposePos(blockPos, gridIdx, foxelIdx)) {
    return H_GET_OOB;
  }

  uint brickPtr = _H_getBrickPtrRepr(tree, gridIdx);
  uint highBit = 1u << 15u;
  if ((brickPtr & highBit) != 0u) {
    // Composite
    uint compositeIdx = brickPtr & ~highBit;
    uint foxelOffset = compositeIdx * TREE_FOXELS_PER_BRICK + foxelIdx;
    foxel = _H_index(tree, TREE_BRICKS_BYTES + foxelOffset);
    return H_GET_FOXEL;
  } else {
    // Solid brick
    foxel = brickPtr & 0xffu;
    return (foxel == 0u ? H_GET_AIR_BRICK : H_GET_FOXEL);
    // return H_GET_FOXEL;
  }
}

struct H_Hit {
  uint foxel;
  vec4 normal;
};

bool H_raycast(sampler2D tree, vec4 start, vec4 dir, out H_Hit hit) {
  ivec4 signums = ivec4(sign(dir));
  // only mutable thing!!
  ivec4 cursor = ivec4(floor(start)) + ivec4(lessThan(dir, vec4(0)));

  ivec4 cursorOffset = -1 * ivec4(lessThan(dir, vec4(0)));

  vec4 dirRecip = 1.0 / dir;
  vec4 slopeSomething = -start * dirRecip;
  // Prevent infinities
  vec4 nonZeroDirs = vec4(notEqual(dir, vec4(0.0)));
  vec4 zeroToSuperBig = 999999.0 * vec4(equal(dir, vec4(0.0)));
  
  bool lastWasAirBrick = false;
  for(uint i = 0u; i < 32u; i++) {
    // TODO: figure out how to skip empty foxels
    float stride = 1.0;
    // float stride = lastWasAirBrick ? float(TREE_FOXELS_ACROSS_BRICK) : 1.0;

    vec4 exitPoses = vec4(cursor) + vec4(signums) * stride;
    vec4 rawExitTimes = fma(exitPoses, dirRecip, slopeSomething);
    vec4 exitTimes = rawExitTimes * nonZeroDirs + zeroToSuperBig;
    // auugh
    float exitTime = min(exitTimes.x, min(exitTimes.y, min(exitTimes.z, exitTimes.w)));
    if (exitTime < 0.0) {
      hit.foxel = 5u;
      return true;
    }
    // epsilon equal
    ivec4 minTimeMap = ivec4(lessThan(
      abs(exitTimes - exitTime),
      vec4(0.00001)
    ));
    ivec4 cursorInc = minTimeMap * signums;

    cursor += cursorInc;
    lastWasAirBrick = false;
    
    uint foxel;
    uint res = H_get(tree, cursor + cursorOffset, foxel);
    if (res == H_GET_OOB) {
      return false;
    } else if (res == H_GET_AIR_BRICK) {
      lastWasAirBrick = true;
      // fallthrough
    } else if (res == H_GET_FOXEL && foxel != 0u) {
      hit.foxel = foxel;
      hit.normal = -vec4(cursorInc);
      return true;
    }
  }

  return false;
}
