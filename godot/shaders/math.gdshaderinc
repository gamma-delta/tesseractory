struct Bivec4 {
	float xy;
	float xz;
	float xw;
	float yz;
	float yw;
	float zw;
}

struct Rotor4 {
	float s;
	Bivec4 bv;
	float p;
}

Rotor4 r4_rotvec(const Rotor4 a, const vec4 v) {
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

	let x = (
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
	let y = (
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
	let z = (
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
	let w = (
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
