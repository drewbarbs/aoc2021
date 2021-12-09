import argparse
import functools
from collections import deque

def get_neighbors(mat, r, c):
    rows, cols = len(mat), len(mat[0])
    return [(r1, c1)
            for r1, c1 in [(r-1, c), (r+1, c), (r, c-1), (r, c+1)]
            if 0 <= r1 < rows and 0 <= c1 < cols]

def get_low_points(mat):
    rows, cols = len(mat), len(mat[0])
    low_points = set()
    for r in range(rows):
        for c in range(cols):
            neighbor_vals = [mat[r1][c1] for r1, c1 in get_neighbors(mat, r, c)]
            if mat[r][c] < min(neighbor_vals):
                low_points.add((r, c))
    return low_points

def get_basin(mat, r, c):
    # do a BFS to identify points in the basin
    basin = []
    queue = deque([(r, c)])
    visited = {(r, c)}
    while queue:
        r, c = queue.popleft()
        elt = mat[r][c]
        if elt != 9:
            basin.append(elt)
            for neighbor in get_neighbors(mat, r, c):
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)
    return basin


def main():
    parser = argparse.ArgumentParser(description='Solution for day 9 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = f.read().splitlines()

    mat = [[int(digit) for digit in line] for line in lines if line.strip()]

    low_points = get_low_points(mat)
    print('Part 1:', sum(mat[r][c] for r, c in low_points) + len(low_points))

    basins = [get_basin(mat, r, c) for r,c in low_points]
    basins.sort(key=lambda b: len(b))
    print('Part 2', functools.reduce(lambda acc, b: acc*len(b), basins[-3:], 1))


if __name__ == '__main__':
    main()
