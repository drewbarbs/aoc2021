import argparse
import math
import re

import numpy as np
from numpy.linalg import inv
from scipy.spatial import distance_matrix


SCANNER_HEADER_PAT = re.compile('--- scanner ([0-9]+) ---')


def have_correspondence(dists_a, dists_b):
    for k, da in enumerate(dists_a):
        for l, db in enumerate(dists_b):
            # We're told overlapping scanners will have at least 12
            # points in common, which means that for corresponding
            # points between two scanners, there will be the same 11
            # (or more) inter-point distances in the corresponding
            # columns of the distance matrix. Every column of every
            # matrix has a "0" entry in common, so we're looking for
            # >= 12
            if len(set(da).intersection(set(db))) >= 12:
                return (k, l)


def parse_input(input_):
    scanners = []

    for line in input_.strip().splitlines():
        if not line.strip():
            continue

        if SCANNER_HEADER_PAT.match(line):
            scanners.append([])
        else:
            scanner = scanners[-1]
            scanner.append(tuple(map(int, line.split(','))))

    return [np.array(s) for s in scanners]


def main():
    parser = argparse.ArgumentParser(description='Solution for day 19 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()
    with open(args.input_file, 'r', encoding='utf8') as f:
        input_ = f.read()

    scanners = parse_input(input_)
    scanner_dists = [distance_matrix(s, s) for s in scanners]

    edges = [[] for _ in scanners]

    for i in range(len(scanners)):
        for j in range(i+1, len(scanners)):
            dists_a = scanner_dists[i]
            dists_b = scanner_dists[j]
            if corresp := have_correspondence(dists_a, dists_b):
                da = dists_a[corresp[0]]
                db = dists_b[corresp[1]]
                da_map = {d: k for k, d in enumerate(da) if d != 0}
                pt_map = {da_map[d]: l for l, d in enumerate(db) if d in da_map}
                edges[i].append((j, pt_map))
                edges[j].append((i, {b: a for a, b in pt_map.items()}))


    # do a DFS to map all the scanners to each other
    stack = [(0, [], np.eye(4))]
    distinct_points = set()
    visited = {0}
    scanner_positions = [None] * len(scanners)
    while stack:
        src_scanner, path, transform_to_0 = stack.pop()
        scanner_positions[src_scanner] = transform_to_0[:3, 3].round(0).astype(int)

        pts = np.hstack((scanners[src_scanner], np.ones(scanners[src_scanner].shape[0]).reshape((-1, 1))))
        transformed = (transform_to_0 @ pts.T).T
        for p in transformed:
            normalized = (p/p[3]).round(0).astype(int)
            final = tuple(normalized)[:3]
            distinct_points.add(final)

        for dst_scanner, corresps in edges[src_scanner]:
            if dst_scanner not in visited:
                # find transform from dst to src
                mappings = list(corresps.items())[:4]
                src_mat = np.hstack((np.array(list(scanners[src_scanner][k] for k, _ in mappings)), np.ones(4).reshape((4, 1))))
                dst_mat = np.hstack((np.array(list(scanners[dst_scanner][k] for _, k in mappings)), np.ones(4).reshape((4, 1))))

                # X^TB^T = A^T
                transform = src_mat.T @ inv(dst_mat.T)
                stack.append((dst_scanner, path + [src_scanner], transform_to_0 @ transform))
                visited.add(dst_scanner)

    scanner_positions = np.array(scanner_positions)
    max_manhattan_dist = int(distance_matrix(scanner_positions, scanner_positions, p=1).max())
    print('Part 1: {}'.format(len(distinct_points)))
    print('Part 2: {}'.format(max_manhattan_dist))


if __name__ == '__main__':
    main()
