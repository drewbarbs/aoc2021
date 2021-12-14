import argparse
import copy
import itertools
from collections import deque
from functools import lru_cache

def advance(mat):
    flash_queue = deque()
    for r, row in enumerate(mat):
        for c, entry in enumerate(row):
            mat[r][c] += 1
            if mat[r][c] >= 10:
                flash_queue.append((r, c))

    flashed = set(flash_queue)
    while flash_queue:
        r, c = flash_queue.popleft()
        for nr, nc in itertools.product((r-1, r, r+1), (c-1, c, c+1)):
            if (nr, nc) == (r, c) or not ((0 <= nr < len(mat)) and (0 <= nc < len(mat[0]))):
                continue
            mat[nr][nc] += 1
            if mat[nr][nc] >= 10 and (nr, nc) not in flashed:
                flash_queue.append((nr, nc))
                flashed.add((nr, nc))

    for (r, c) in flashed:
        mat[r][c] = 0

    return len(flashed)

def main():
    parser = argparse.ArgumentParser(description='Solution for day 11 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = [l.strip() for l in f.read().splitlines() if l.strip()]

    mat = [[int(c) for c in line] for line in lines]

    cache = {}
    total = 0
    for _ in range(100):
        total += advance(mat)

    print('Total', total)

    mat = [[int(c) for c in line] for line in lines]
    entry_count = len(mat) * len(mat[0])
    iterations = 0
    while True:
        inc = advance(mat)
        iterations += 1
        if inc == entry_count:
            print(iterations)
            break


if __name__ == '__main__':
    main()
