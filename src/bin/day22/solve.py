import argparse
import bisect
import re

LINE_REGEX = re.compile('^(on|off) x=([-0-9]+)\\.\\.([-0-9]+),y=([-0-9]+)\\.\\.([-0-9]+),z=([-0-9]+)\\.\\.([-0-9]+)$')


def get_intersection(intervals, query):
    intersection = []
    if not intervals:
        return intersection

    lo, hi = query

    idx = bisect.bisect_left(intervals, ((lo, float('-inf')), []))
    if idx > 0 and intervals[idx-1][0][1] >= lo:
        intersection.append(intervals[idx-1])

    # add intervals this one extends past
    for i in range(idx, len(intervals)):
        if hi >= intervals[i][0][0]:
            intersection.append(intervals[i])
        else:
            break

    return intersection


def part1(instrs):
    turned_on = []

    for (action, (x_min, x_max), (y_min, y_max), (z_min, z_max)) in instrs:
        if action == 'on':
            intersecting_x = get_intersection(turned_on, (x_min, x_max))
            if not intersecting_x:
                insert = bisect.bisect_left(turned_on, ((x_min, x_max), []))
                turned_on.insert(insert, ((x_min, x_max), [((y_min, y_max), [((z_min, z_max), [])])]))
            else:
                while intersecting_x:
                    ((fst_min_x, fst_max_x), fst_squares) = intersecting_x[0]
                    if fst_min_x == x_min and fst_max_x == x_max:
                        # now we just need to insert new squares into fst_squares
                        pass

                    # split the range
                    pass


def parse_input(s):
    instrs = []
    for l in s.splitlines():
        m = LINE_REGEX.match(l)
        tup = (m.group(1), (int(m.group(2)), int(m.group(3))),
               (int(m.group(4)), int(m.group(5))),
               (int(m.group(6)), int(m.group(7))))
        instrs.append(tup)
    return instrs


def main():
    parser = argparse.ArgumentParser(description='Solution for day 22 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file, 'r', encoding='utf8') as f:
        instrs = parse_input(f.read())

    part1(instrs)


if __name__ == '__main__':
    main()
