shader_type canvas_item;
render_mode unshaded;

#include "math.gdshaderinc"
#include "hexadecitree.gdshaderinc"

void fragment() {
  COLOR = vec4(UV.x, UV.y, 0.0, 1.0);
}
