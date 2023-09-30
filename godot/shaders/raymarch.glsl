#[compute]
#version 460

layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

layout(set = 0, binding = 0) uniform Consts {
  uint TREE_DEPTH;
  uint TREE_MAX_COORD;
  uint TREE_MIN_COORD;
};

layout(set = 0, binding = 1) restrict buffer Hexadecitree tree;
layout(rgba32f, set = 0, binding = 2) restrict uniform image2D imgOutput;

// =====
// GEOMETRIC ALGEBRA
// =====

struct Bivec4 {
	float xy;
	float xz;
	float xw;
	float yz;
	float yw;
	float zw;
};

struct Rotor4 {
	float s;
	Bivec4 bv;
	float p;
};

vec4 r4_rotvec(const Rotor4 self, const vec4 a) {
	// Write out the values like this to make it easier to
	// copy-paste Joe's code
	float s = self.s;
	Bivec4 b = self.bv;
	float pxyzw = self.p;

	float s2 = s * s;
	float bxy2 = b.xy * b.xy;
	float bxz2 = b.xz * b.xz;
	float bxw2 = b.xw * b.xw;
	float byz2 = b.yz * b.yz;
	float byw2 = b.yw * b.yw;
	float bzw2 = b.zw * b.zw;
	float bxyzw2 = pxyzw * pxyzw;

	float x = (
	      2.0 * a.w * b.xw * s
	    + 2.0 * a.w * b.xy * b.yw
	    + 2.0 * a.w * b.xz * b.zw
	    + 2.0 * a.w * b.yz * pxyzw
	    - a.x * bxw2
	    - a.x * bxy2
	    - a.x * bxz2
	    + a.x * byw2
	    + a.x * byz2
	    + a.x * bzw2
	    - a.x * bxyzw2
	    + a.x * s2
	    - 2.0 * a.y * b.xw * b.yw
	    + 2.0 * a.y * b.xy * s
	    - 2.0 * a.y * b.xz * b.yz
	    + 2.0 * a.y * b.zw * pxyzw
	    - 2.0 * a.z * b.xw * b.zw
	    + 2.0 * a.z * b.xy * b.yz
	    + 2.0 * a.z * b.xz * s
	    - 2.0 * a.z * b.yw * pxyzw
	);
	float y = (
	    - 2.0 * a.w * b.xw * b.xy
	    - 2.0 * a.w * b.xz * pxyzw
	    + 2.0 * a.w * b.yw * s
	    + 2.0 * a.w * b.yz * b.zw
	    - 2.0 * a.x * b.xw * b.yw
	    - 2.0 * a.x * b.xy * s
	    - 2.0 * a.x * b.xz * b.yz
	    - 2.0 * a.x * b.zw * pxyzw
	    + a.y * bxw2
	    - a.y * bxy2
	    + a.y * bxz2
	    - a.y * byw2
	    - a.y * byz2
	    + a.y * bzw2
	    - a.y * bxyzw2
	    + a.y * s2
	    + 2.0 * a.z * b.xw * pxyzw
	    - 2.0 * a.z * b.xy * b.xz
	    - 2.0 * a.z * b.yw * b.zw
	    + 2.0 * a.z * b.yz * s
	);
	float z = (
	    - 2.0 * a.w * b.xw * b.xz
	    + 2.0 * a.w * b.xy * pxyzw
	    - 2.0 * a.w * b.yw * b.yz
	    + 2.0 * a.w * b.zw * s
	    - 2.0 * a.x * b.xw * b.zw
	    + 2.0 * a.x * b.xy * b.yz
	    - 2.0 * a.x * b.xz * s
	    + 2.0 * a.x * b.yw * pxyzw
	    - 2.0 * a.y * b.xw * pxyzw
	    - 2.0 * a.y * b.xy * b.xz
	    - 2.0 * a.y * b.yw * b.zw
	    - 2.0 * a.y * b.yz * s
	    + a.z * bxw2
	    + a.z * bxy2
	    - a.z * bxz2
	    + a.z * byw2
	    - a.z * byz2
	    - a.z * bzw2
	    - a.z * bxyzw2
	    + a.z * s2
	);
	float w = (
	    - a.w * bxw2
	    + a.w * bxy2
	    + a.w * bxz2
	    - a.w * byw2
	    + a.w * byz2
	    - a.w * bzw2
	    - a.w * bxyzw2
	    + a.w * s2
	    - 2.0 * a.x * b.xw * s
	    + 2.0 * a.x * b.xy * b.yw
	    + 2.0 * a.x * b.xz * b.zw
	    - 2.0 * a.x * b.yz * pxyzw
	    - 2.0 * a.y * b.xw * b.xy
	    + 2.0 * a.y * b.xz * pxyzw
	    - 2.0 * a.y * b.yw * s
	    + 2.0 * a.y * b.yz * b.zw
	    - 2.0 * a.z * b.xw * b.xz
	    - 2.0 * a.z * b.xy * pxyzw
	    - 2.0 * a.z * b.yw * b.yz
	    - 2.0 * a.z * b.zw * s
	);
	return vec4(x, y, z, w);
}


// =====
// TREE
// =====


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

// Rather than recurse, use a loop.
// Note that the out idx is bytes within the u32
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


// =====
// THE ACTUAL SHADER
// =====

void main() {
  ivec2 texelCoord = ivec2(gl_GlobalInvocationID.xy);

  vec4 value = vec4(
    float(texelCoord.x)/(gl_NumWorkGroups.x),
    float(texelCoord.y)/(gl_NumWorkGroups.y),
    0.0, 0.0
  );
  
  imageStore(imgOutput, texelCoord, value);
}
