shader_type canvas_item;
render_mode unshaded;

#include "math.gdshaderinc"
#include "hexadecitree.gdshaderinc"

uniform sampler2D tree : filter_nearest;
uniform sampler2D foxelPalette : filter_nearest;

uniform vec4 playerPos;

uniform float focalDist;
uniform float fov;

void fragment() {
  vec2 centered = UV - 0.5;
  vec2 dir2d = centered * fov * TEXTURE_PIXEL_SIZE;
  // vec4 rayDir = normalize(vec4(-dir2d.y, focalDist, -dir2d.x, 0.0));
  vec4 rayDir = vec4(0.0, 1.0, 0.0, 0.0);

  H_Hit hit;
  if (H_raycast(tree, playerPos, rayDir, hit)) {
    // Temp foxel color lookup
    vec2 coords = vec2(
      float(hit.foxel % 16u) / 16.0,
      float(hit.foxel / 16u) / 16.0
    );
    COLOR = texture(foxelPalette, coords);
  } else {
    COLOR = vec4(UV.x, UV.y, 0.0, 1.0);
  }
}
