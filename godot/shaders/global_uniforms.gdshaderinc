#ifndef CODING_LIKE_ITS_1989
#define CODING_LIKE_ITS_1989

global uniform uint TREE_COMPOSITE_BRICK_COUNT;

global uniform uint TREE_FOXELS_ACROSS_BRICK;
global uniform uint TREE_FOXELS_PER_BRICK;
global uniform uint TREE_BRICKS_ACROSS_WORLD;
global uniform uint TREE_BRICKS_BYTES;

global uniform int TREE_MIN_COORD;
global uniform int TREE_MAX_COORD;

global uniform uint TREE_TRANSFER_IMG_SIZE;

// https://github.com/godotengine/godot/issues/57841
// Apparently we can bytemuck this and maybe save some perf?
uniform usampler2D TREE_TEXTURE : filter_nearest;

#endif
