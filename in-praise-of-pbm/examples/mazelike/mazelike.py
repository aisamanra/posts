import random

# we rely on these being odd later on
WIDTH, HEIGHT = 51, 31

# set up all the pixels as black for the time being
PIXELS = {(x,y): 1 for x in range(WIDTH) for y in range(HEIGHT)}

# for every pixel in the image
for (x, y) in PIXELS:
    # this effectively makes a grid of white pixels separated by black
    # lines
    if x % 2 == 1 and y % 2 == 1:
        PIXELS[(x, y)] = 0
        # but at each grid line, choose to make a random line going
        # either right or down
        if random.random() > 0.5:
            PIXELS[(x+1, y)] = 0
        else:
            PIXELS[(x, y+1)] = 0

    # also just make the edges black so we don't have white trails
    # going off the side.
    if x == WIDTH - 1 or y == HEIGHT - 1:
        PIXELS[(x, y)] = 1

# print the PBM file
print('P1')
print(f'{WIDTH} {HEIGHT}')
for y in range(HEIGHT):
    # we could do this all on one line, but this is nice for debugging
    for x in range(WIDTH):
        print(f'{PIXELS[x,y]}', end=' ')
    print()
