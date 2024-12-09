from collections import defaultdict
import math

r = """
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#."""

f = """
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"""
grid = []
with open("./input.txt", "r") as f:
    grid = [list(line.strip()) for line in f]


#grid = [list(row) for row in f.strip().split('\n')]
rows = len(grid)
cols = len(grid[0])

node_map = defaultdict(list)

def in_grid(x, y):
    return x >= 0 and x < cols and y >= 0 and y < rows

def get_antinodes(x1, y1, x2, y2):
    dx = x2 - x1
    dy = y2 - y1

    nodes = []

    i = 0
    while True:
        c_x = x1 - dx * i
        c_y = y1 - dy * i

        if (in_grid(c_x, c_y)):
            print("in grid 1")
            nodes.append((c_x, c_y))
        else:
            break
        i += 1
       
    i = 0

    while True:
        c_x = x2 + dx * i
        c_y = y2 + dy * i

        if (in_grid(c_x, c_y)):
            print("in grid 2")
            nodes.append((c_x, c_y))
        else:
            break

        i += 1
       
    return nodes

for row_idx, row in enumerate(grid):
    for col_idx, char in enumerate(row):
        if (char != '.'):
            node_map[char].append((row_idx, col_idx))
        
antinodes = set()

for k, v in node_map.items():
    print(f"{k} - count:{len(node_map[k])} - nodes: {v}")
    for idx, node in enumerate(v):
        for pair_idx, pair_node in enumerate(v):
            if pair_idx != idx:
                nodes = get_antinodes(node[1], node[0], pair_node[1], pair_node[0])
                for antinode in nodes:
                    antinodes.add(antinode)
               
print(len(antinodes))
