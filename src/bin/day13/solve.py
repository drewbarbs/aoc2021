import argparse
import re

def parse_input(lines):
    dots = set()
    l_iter = iter(lines)

    for l in l_iter:
        if not l.strip():
            # hit the end of the coords
            break

        x, y = l.split(',')
        dots.add((int(x), int(y)))

    folds = []
    for l in l_iter:
        m = re.match('^fold along ([xy])=([0-9]+)', l)
        folds.append((m.group(1), int(m.group(2))))

    return dots, folds


def do_fold(dots, fold):
    dots = dots.copy()
    axis, v = fold
    if axis == 'x':
        discard = [dot for dot in dots if dot[0] > v]
        dots.difference_update(discard)
        to_add = [(v - (x - v), y) for x, y in discard]
        minx = min(t[0] for t in to_add)
        dots.update(to_add)
        if minx < 0:
            dots = {(x + abs(minx), y) for x, y in dots}
    else:
        discard = [dot for dot in dots if dot[1] > v]
        dots.difference_update(discard)
        to_add = [(x, v - (y - v)) for x, y in discard]
        miny = min(t[1] for t in to_add)
        dots.update(to_add)
        if miny < 0:
            dots = {(x, y + abs(miny)) for x, y in dots}

    return dots

def show(dots):
    max_x = max(t[0] for t in dots) + 1
    max_y = max(t[1] for t in dots) + 1

    grid = [[' ']*max_x for _ in range(max_y)]
    for r, c in dots:
        grid[c][r] = '#'

    print('\n'.join(''.join(row) for row in grid))


def part1(dots, folds):
    dots = do_fold(dots, folds[0])

    return len(dots)

def part2(dots, folds):
    for fold in folds:
        dots = do_fold(dots, fold)

    show(dots)

def main():
    parser = argparse.ArgumentParser(description='Solution for day 13 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = f.read().splitlines()

    dots, folds = parse_input(lines)
    print(part1(dots, folds))
    part2(dots, folds)


if __name__ == '__main__':
    main()
