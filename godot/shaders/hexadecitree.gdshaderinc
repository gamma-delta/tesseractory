#include "global_uniforms.gdshaderinc"

const uint _H_HIGH_BIT = 1u << 15u;

// https://stackoverflow.com/questions/14997165/fastest-way-to-get-a-positive-modulo-in-c-c
#define H_rem(i, n) ( uint((i % int(n) + int(n)) % int(n)) )
// uint H_rem(int i, uint n) {
//   int ni = int(n);
//   return uint((i % ni + ni) % ni);
// }

// Return the idx'th byte of the tree.
// https://github.com/godotengine/godot/issues/57841
// due to problems this is more difficult than it has to be.
uint _H_index(uint idx) {
  uint width = uint(textureSize(TREE_TEXTURE, 0).x);
  uint idxOver4 = idx / 4u;
  uint subByte = idx % 4u;

  ivec2 coords = ivec2(
    int(idxOver4 % width),
    int(idxOver4 / width)
  );
  uint bytes = texelFetch(TREE_TEXTURE, coords, 0).r;
  // uint shift = 3u - subByte;
  uint shift = subByte;
  return (bytes >> (shift * 8u)) & 0xffu;
}

uint _H_getBrickPtrRepr(uint idx) {
  uint width = uint(textureSize(TREE_TEXTURE, 0).x);
  uint shortIdx = idx / 2u;
  uint subByte = idx % 2u;

  ivec2 coords = ivec2(
    int(shortIdx % width),
    int(shortIdx / width)
  );
  uint bytes = texelFetch(TREE_TEXTURE, coords, 0).r;
  return (bytes >> (subByte * 8u)) & 0xffffu;
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
uint H_get(ivec4 blockPos, out uint foxel) {
  uint gridIdx, foxelIdx;
  if (!_H_decomposePos(blockPos, gridIdx, foxelIdx)) {
    return H_GET_OOB;
  }

  uint brickPtr = _H_getBrickPtrRepr(gridIdx);
  uint highBit = 1u << 15u;
  if ((brickPtr & highBit) != 0u) {
    // Composite
    uint compositeIdx = brickPtr & ~highBit;
    uint foxelOffset = compositeIdx * TREE_FOXELS_PER_BRICK + foxelIdx;
    foxel = _H_index(TREE_BRICKS_BYTES + foxelOffset);
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
  vec4 texpos;
};

// [1] https://ramakarl.com/pdfs/2016_Hoetzlein_GVDB.pdf
// [2] https://blog.balintcsala.com/posts/voxel-tracing/
// https://www.cs.cornell.edu/courses/cs4620/2013fa/lectures/03raytracing1.pdf
// to step across empty voxels, just pretend i'm doing a standard box-
// intersection test every time
bool H_raycast(vec4 start, vec4 direction, out H_Hit hit) {
  vec4 stepSizes = 1.0 / direction;
  vec4 stepSigns = sign(direction);
  // only mutable thing!
  vec4 curPos = start;

  bool prevWasAirBrick = false;

  for(uint i = 0u; i < 64u; i++) {
    vec4 brickSize = prevWasAirBrick 
        ? vec4(float(TREE_FOXELS_ACROSS_BRICK)) : vec4(1.0);
    // gdshaders don't have piecewise ternary, but they do have this.
    // i think it goes false, true though
    vec4 snapAwayFromRay = mix(
        ceil(curPos), floor(curPos),
        greaterThan(direction, vec4(0.0)));
    vec4 voxelEnds = 
        (floor(snapAwayFromRay / brickSize) + stepSigns) * brickSize;
    // box intersection
    vec4 minTimes = abs((voxelEnds - curPos) * stepSizes);
    float minTime = 
        min(minTimes.x, min(minTimes.y, min(minTimes.z, minTimes.w)));

    curPos += direction * minTime;

    ivec4 foxelPos = ivec4(floor(curPos + direction * 0.00001));
    uint foxel;
    uint res = H_get(foxelPos, foxel);
    if (res == H_GET_OOB) {
      return false;
    } else if (res == H_GET_FOXEL && foxel != 0u) {
      ivec4 steppedInAxisMask = ivec4(equal(minTimes, vec4(minTime)));

      hit.foxel = foxel;
      hit.normal = vec4(1.0, 0.0, 0.0, 0.0);
      hit.texpos = curPos;
      return true;
    }
    prevWasAirBrick = res == H_GET_AIR_BRICK;
  }

  return false;
}
