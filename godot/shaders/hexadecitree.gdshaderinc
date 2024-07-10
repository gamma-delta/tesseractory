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
bool H_raycast(vec4 start, vec4 dir, out H_Hit hit) {
  ivec4 stepDir = ivec4(sign(dir));
  vec4 nextDist = (vec4(stepDir) * 0.5 + 0.5 - fract(start)) / dir;

  // only mutable things!!
  vec4 curPos = start;
  int stride = 1;

  uint prevBrickIdx = 0xDEADu;
  uint _scratch = 69420u;
  bool firstBrick = true;
  bool prevWasAirBrick = false;

  /*
  Herein lies the rub:
  When stepping by 1, the foxel scanned goes up either by 0 or 1 in each dir
  But when stepping by more than 1, the number of foxels traversed by each
  can vary.
  For example
    +---+
    |   |
    |  .X..>
   .X.. |
>.. +---+

the line goes across 5, and up 1.
  */
 
  for(uint i = 0u; i < 64u; i++) {
    vec4 stepSizes = float(stride) / abs(dir);
    float closestDist = min(nextDist.x, min(nextDist.y, min(nextDist.z, nextDist.w)));
    vec4 amountSteppedPerAxis = closestDist / dir;
    curPos += dir * closestDist;
    ivec4 foxelCursor = ivec4(floor(curPos));
    
    uint prevPrevBrickIdx = prevBrickIdx;
    _H_decomposePos(foxelCursor, prevBrickIdx, _scratch);
    if (prevPrevBrickIdx != 0xDEADu && prevPrevBrickIdx != prevBrickIdx) {
      firstBrick = false;
    }
    
    uint foxel;
    uint res = H_get(foxelCursor, foxel);
    if (res == H_GET_OOB) {
      return false;
    } else if (res == H_GET_FOXEL && foxel != 0u) {
      // ivec4 steppedInAxisMask = ivec4(lessThan(
      //   abs(nextDist - closestDist),
      //   vec4(0.00001)
      // ));

      hit.foxel = foxel;
      hit.normal = vec4(1.0, 0.0, 0.0, 0.0);
      hit.texpos = curPos;
      return true;
    }
    prevWasAirBrick = res == H_GET_AIR_BRICK;

    // stride = (!firstBrick && prevWasAirBrick)
    //   ? int(TREE_FOXELS_ACROSS_BRICK)
    //   : 1;
    // reset to boundary
    nextDist -= closestDist;
    nextDist += stepSizes * amountSteppedPerAxis * float(stride);
  }

  return false;
}
