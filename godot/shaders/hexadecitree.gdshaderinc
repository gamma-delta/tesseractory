#include "global_uniforms.gdshaderinc"

const uint _H_HIGH_BIT = 1u << 31u;

struct Hexadecitree {
  uint[65536] branches;
  uint[8192] foxels;
};

uint _H_stepDownOne(inout ivec4 pos, bool depthZero) {
  uint zeroIdx = uint(pos.x >= 0)
      | uint(pos.y >= 0) << 1u
      | uint(pos.z >= 0) << 2u
      | uint(pos.w >= 0) << 3u;
  ivec4 zeroPos = abs(pos);
  
  uint notZeroIdx = uint((pos.x & 1) != 0)
      | uint((pos.y & 1) != 0) << 1u
      | uint((pos.z & 1) != 0) << 2u
      | uint((pos.w & 1) != 0) << 3u;
  ivec4 notZeroPos = pos / 2;
  
  pos = depthZero ? zeroPos : notZeroPos;
  return depthZero ? zeroIdx : notZeroIdx;
}

uint _H_stepDownPos(inout ivec4 pos, bool depthZero) {
  uint idx1 = _H_stepDownOne(pos, depthZero);
  uint idx2 = _H_stepDownOne(pos, false);
  return (idx2 << 4u) | idx1;
}

// Rather than recurse, use a loop
bool _H_get(Hexadecitree self, ivec4 pos, out uint foxelIdx) {
  if (any(greaterThan(pos, ivec4(1023))) || any(lessThan(pos, ivec4(-1024)))) {
    return false;    
  }
  
  uint treeRef = 0u; // start at the root
  uint depth = 0u;
  for (;;) {
    // mutates pos!
    uint childIdx = _H_stepDownPos(pos, depth == 0u);
    uint treeRepr = self.branches[treeRef];
    
    bool highBitSet = (treeRepr & _H_HIGH_BIT) != 0u;    
    if (depth == TREE_DEPTH - 1u) {
      // better find a leaf node here
      if (!highBitSet) {
        // oh dear. invalid state, expected a leaf
        return false;
      }
      uint foxelSpanIdx = treeRepr & 0xffffu;
      foxelIdx = foxelSpanIdx * 256u + childIdx;
      return true;
    } else {
      if (treeRepr == 0u || highBitSet) {
        // if 0, then this is empty.
        // if high bit, then invalid state, did not expect a leaf
        // in both cases, shortcut out.
        return false;
      }
      // now is the branch idx, which is displayed as is
      treeRef = treeRepr + childIdx;
    }
  
    depth++;
  }
  
  // unreachable I hope
  return false;
}
