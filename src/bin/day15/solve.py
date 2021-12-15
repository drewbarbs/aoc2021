import argparse
import heapq


def i2c(N, M, idx):
    return (idx//M, idx % M)


def c2i(N, M, r, c):
    return r*M + c

def neighbors(N, M, r, c):
    return [(nr, nc) for nr, nc in ((r-1, c), (r+1, c), (r, c-1), (r, c+1))
            if 0 <= nr < N and 0 <= nc < M]

class PriQ:
    def __init__(self):
        self.pq = []
        self.entry_finder = {}

    def remove(self, idx):
        entry = self.entry_finder.pop(idx)
        entry[-1] = None

    def add(self, idx, dist):
        if idx in self.entry_finder:
            remove(idx)
        entry = [dist, idx]
        self.entry_finder[idx] = entry
        heapq.heappush(self.pq, entry)

    def pop(self):
        while self.pq:
            dist, idx = heapq.heappop(self.pq)
            if idx is not None:
                del self.entry_finder[idx]
                return dist, idx
        raise KeyError('pop from empty priq')


def dijkstra(mat, source_node, target_node):
    N = len(mat)
    M = len(mat[0])
    dist_to = [-1] * (N * M)
    priq = PriQ()

    priq.add(0, 0)
    while priq.pq:
        try:
            dist, idx = priq.pop()
        except KeyError:
            break
        dist_to[idx] = dist
        if idx == target_node:
            break

        r, c = i2c(N, M, idx)
        for nr, nc in neighbors(N, M, r, c):
            entry_cost = mat[nr][nc]
            nidx = c2i(N, M, nr, nc)
            cur_dist = dist_to[nidx]
            if cur_dist == -1 or (entry_cost + dist < cur_dist):
                dist_to[nidx] = entry_cost + dist
                priq.add(nidx, dist_to[nidx])

    return dist_to


def enlarge(mat, n):
    CUR_N = len(mat)
    CUR_M = len(mat[0])

    new_mat = [[0] * CUR_M * n for _ in range(CUR_N * n)]
    for r in range(CUR_N * n):
        for c in range(CUR_M * n):
            source_r, source_c = r % CUR_N, c % CUR_M
            dist_r, dist_c = (r - source_r) // CUR_N, (c - source_c) // CUR_M
            new_val = mat[source_r][source_c] + dist_r + dist_c
            new_mat[r][c] = ((new_val - 1) % 9) + 1

    return new_mat


def main():
    parser = argparse.ArgumentParser(description='Solution for day 15 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = [l.strip() for l in f.readlines()]

    mat = [[int(c) for c in line] for line in lines]
    N = len(mat)
    M = len(mat[0])

    dists = dijkstra(mat, 0, N*M-1)
    print(dists[-1])

    new_mat = enlarge(mat, 5)
    new_N = len(new_mat)
    new_M = len(new_mat[0])
    dists = dijkstra(new_mat, 0, new_N * new_M - 1)
    print(dists[-1])


if __name__ == '__main__':
    main()
