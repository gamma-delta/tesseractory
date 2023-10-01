shader_type canvas_item;
render_mode unshaded;

#include "math.gdshaderinc"
#include "hexadecitree.gdshaderinc"

uniform sampler2D tree : filter_nearest;
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
  if (H_raycast(tree, playerPos + rayDir * focalDist, rayDir, hit)) {
    // Temp foxel color lookup
    vec2 coords = vec2(
      float(hit.foxel % 16u) / 16.0,
      float(hit.foxel / 16u) / 16.0
    );
    vec4 col = texture(foxelPalette, coords);
    float light = dot(-hit.normal, normalize(
      vec4(-0.5, 0.4, 0.2, 0.1)));
    COLOR = col * clamp(light, 0.5, 1.0);
  } else {
    COLOR = vec4(UV.x, UV.y, 0.0, 1.0);
  }
}
