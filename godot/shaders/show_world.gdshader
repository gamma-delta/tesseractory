shader_type canvas_item;
render_mode unshaded;

#include "math.gdshaderinc"
#include "hexadecitree.gdshaderinc"

uniform sampler2D tree : filter_nearest;
uniform sampler2D foxelPalette : filter_nearest;

uniform vec4 playerPos;

void fragment() {
  uint foxel;
  if (H_get(tree, ivec4(playerPos), foxel)) {
    vec2 coords = vec2(
      float(foxel % 16u) / 16.0,
      float(foxel / 16u) / 16.0
    );
    COLOR = texture(foxelPalette, coords);
  } else {
    COLOR = vec4(UV.x, UV.y, 0.0, 1.0);
  }
}
