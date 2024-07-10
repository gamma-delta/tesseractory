shader_type canvas_item;
render_mode unshaded;

#include "math.gdshaderinc"
#include "hexadecitree.gdshaderinc"

uniform sampler2D foxelPalette : filter_nearest;

uniform vec4 playerPos;
uniform float[8] playerLookRaw;

uniform float focalDist;
uniform float fov;
uniform float aspectRatio;

void fragment() {
  vec2 centered = UV - 0.5;
  vec2 dir2d = centered * fov * vec2(aspectRatio, 1.0);
  vec4 rawRayDir = normalize(vec4(-dir2d.y, 1.0, -dir2d.x, 0.0));

  Rotor4 playerLook = R4_decode(playerLookRaw);
  vec4 rayDir = R4_rotvec(playerLook, rawRayDir);

  H_Hit hit;
  if (H_raycast(playerPos + rayDir * focalDist, rayDir, hit)) {
    // Temp foxel color lookup
    vec2 coords = vec2(
      float(hit.foxel % 16u) / 16.0,
      float(hit.foxel / 16u) / 16.0
    );
    vec4 col = texture(foxelPalette, coords);
    // vec4 col = vec4(hit.texpos, 1.0);
    float light = dot(-hit.normal, normalize(
      vec4(-0.5, 0.4, 0.2, 0.1)));
    COLOR = col * clamp(light, 0.8, 1.0);
  } else {
    vec4 flipRay = abs(rayDir);
    vec4 squishRay = clamp(
      8.0 * (flipRay * flipRay * flipRay * flipRay - 1.0) + 1.0,
      0, 1
    );

    vec3 xCol = (1.0 - vec3(1, 0, 0)) * squishRay.x;
    vec3 yCol = (1.0 - vec3(0, 1, 0)) * squishRay.y;
    vec3 zCol = (1.0 - vec3(0, 0, 1)) * squishRay.z;
    vec3 wCol = (1.0 - vec3(1, 0, 1)) * squishRay.w;
    vec3 baseColor = vec3(0.9, 0.9, 0.95);
    COLOR = vec4(clamp(baseColor - xCol - yCol - zCol - wCol, 0, 1), 1);
  }
}
