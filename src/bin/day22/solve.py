import argparse
import functools
import re


LINE_REGEX = re.compile('^(on|off) x=([-0-9]+)\\.\\.([-0-9]+),y=([-0-9]+)\\.\\.([-0-9]+),z=([-0-9]+)\\.\\.([-0-9]+)$')


def get_intersection_interval(a, b):
    astart, aend = a
    bstart, bend = b

    if astart <= bstart and aend >= bstart:
        return bstart, min(aend, bend)
    elif astart > bstart and astart <= bend:
        return astart, min(aend, bend)


def get_intersection_volume(a, b):
    a_xrange, a_yrange, a_zrange = a
    b_xrange, b_yrange, b_zrange = b

    if xoverlap := get_intersection_interval(a_xrange, b_xrange):
        if yoverlap := get_intersection_interval(a_yrange, b_yrange):
            if zoverlap := get_intersection_interval(a_zrange, b_zrange):
                return xoverlap, yoverlap, zoverlap


def get_volume(box):
    return functools.reduce(lambda a, b: a*b, (abs(dim[1] - dim[0]) + 1 for dim in box))


def get_difference_volumes(vol, sub_vol):
    """ Return the parts of vol not in sub_vol """
    if not vol:
        return []

    difference_volumes = []
    if pre_int := get_intersection_interval(vol[0], (float('-inf'), sub_vol[0][0] - 1)):
        difference_volumes.append((pre_int,) + vol[1:])

    if post_int := get_intersection_interval(vol[0], (sub_vol[0][1] + 1, float('inf'))):
        difference_volumes.append((post_int,) + vol[1:])

    intersect_int = get_intersection_interval(vol[0], sub_vol[0])
    for sub_diff in get_difference_volumes(vol[1:], sub_vol[1:]):
        difference_volumes.append((intersect_int,) + sub_diff)

    return difference_volumes


def add_to_volumes(volumes, to_add):
    overlaps = []
    for v in volumes:
        if overlap := get_intersection_volume(v, to_add):
            overlaps.append(v)

    addition_volumes = [to_add]
    for overlap in overlaps:
        addition_volumes = remove_from_volumes(addition_volumes, overlap)

    new_volumes = volumes[:]
    new_volumes.extend(addition_volumes)

    return new_volumes


def do_remove(volumes, removals):
    for n_processed, v in enumerate(volumes):
        for j, removal in enumerate(removals):
            if overlap := get_intersection_volume(v, removal):
                unprocessed_volumes = get_difference_volumes(v, overlap)
                unprocessed_volumes.extend(volumes[n_processed+1:])
                new_removals = removals[:j]
                new_removals.extend(get_difference_volumes(removal, overlap))
                new_removals.extend(removals[j+1:])
                return volumes[:n_processed], unprocessed_volumes, new_removals

    return volumes, [], []


def remove_from_volumes(volumes, to_remove):
    final_volumes = []
    removals = [to_remove]
    unprocessed = volumes
    while removals:
        processed, unprocessed, removals = do_remove(unprocessed, removals)
        final_volumes.extend(processed)

    final_volumes.extend(unprocessed)

    return final_volumes


def run_seq(instrs):
    boxes = []

    for (action, x_range, y_range, z_range) in instrs:
        box = (x_range, y_range, z_range)
        if action == 'on':
            boxes = add_to_volumes(boxes, box)
        else:
            assert action == 'off'
            boxes = remove_from_volumes(boxes, box)

    return sum(get_volume(b) for b in boxes)


def part1(instrs):
    def in_range(r):
        return abs(r[0]) <= 50 and abs(r[1]) <= 50

    instrs = [(act, x_r, y_r, z_r) for act, x_r, y_r, z_r in instrs
              if in_range(x_r) and in_range(y_r) and in_range(z_r)]

    return run_seq(instrs)


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

    print(part1(instrs))
    print(run_seq(instrs))


if __name__ == '__main__':
    main()
