BRICKS_ACROSS_WORLD = 32
FOXELS_ACROSS_BRICK = 8

def rem(i, n):
    return (i % n + i) % n


def decompose1(v):
    foxelPos = rem(v, FOXELS_ACROSS_BRICK)

    rawGridPos = None
    if v >= 0:
        rawGridPos = v // FOXELS_ACROSS_BRICK
    else:
        rawGridPos = v // FOXELS_ACROSS_BRICK - 1

    gridPos = rawGridPos + BRICKS_ACROSS_WORLD // 2
    return (gridPos, foxelPos)

def decompose(pos):
    gridIdx = 0
    foxelIdx = 0
    # scratch

    g, f = decompose1(pos[0])
    gridIdx |= g; gridIdx *= BRICKS_ACROSS_WORLD;
    foxelIdx |= f; foxelIdx *= FOXELS_ACROSS_BRICK;
    g, f = decompose1(pos[1])
    gridIdx |= g; gridIdx *= BRICKS_ACROSS_WORLD;
    foxelIdx |= f; foxelIdx *= FOXELS_ACROSS_BRICK;
    g, f = decompose1(pos[2])
    gridIdx |= g; gridIdx *= BRICKS_ACROSS_WORLD;
    foxelIdx |= f; foxelIdx *= FOXELS_ACROSS_BRICK;
    g, f = decompose1(pos[3])
    gridIdx |= g
    foxelIdx |= f

    return (gridIdx, foxelIdx)

for p in [
    (0, 0, 0, 0),
    (1, 0, 0, 0),
    (0, 1, 0, 0),
    (0, 0, 1, 0),
    (0, 0, 0, 1),
]:
    gridIdx, foxelIdx = decompose(p)
    print(f"{p} -> {gridIdx}, {foxelIdx}")
