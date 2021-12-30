import argparse
import heapq
import itertools
from collections import defaultdict
from typing import Dict, List, Tuple


ENERGY_PER_STEP = {
    'A': 1,
    'B': 10,
    'C': 100,
    'D': 1000,
}


def print_map(map_):
    max_x = max(t[0] for t in map_)
    max_y = max(t[1] for t in map_)

    for y in range(max_y+1):
        for x in range(max_x+1):
            print(map_.get((x, y), ' '), end='')
        print()


def potential_moves(start_pos, amphipod_type, move_count, cur_map, rooms) -> List[Tuple[Tuple[int, int], int]]:
    doorways = [(spots[0][0], spots[0][1] - 1) for spots in rooms.values()]
    other_room_spaces = [s for typ, spaces in rooms.items() if typ != amphipod_type for s in spaces]

    # amphipod can only enter its room if no other type of amphipod is in that room,
    # will go to the furthest-in space
    if any(cur_map[s] in 'ABCD' and cur_map[s] != amphipod_type for s in rooms[amphipod_type]):
        target_spot = None
    else:
        target_spot = max((space for space in rooms[amphipod_type] if cur_map[space] == '.'), default=None)

    if move_count == 1 and target_spot is None:
        return

    visited = {start_pos}
    frontier = [(start_pos, 0)]
    step_energy = ENERGY_PER_STEP[amphipod_type]
    while frontier:
        (x, y), cost = frontier.pop(0)
        for next_pos in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
            if cur_map[next_pos] != '.' or next_pos in visited:
                continue

            frontier.append((next_pos, cost + step_energy))
            visited.add(next_pos)

            if next_pos in other_room_spaces:
                continue

            # can we actually stop here?
            if next_pos == target_spot:
                yield frontier[-1]
            elif move_count == 0 and next_pos not in doorways and next_pos[0] != rooms[amphipod_type][0][0]:
                yield frontier[-1]


def successor_states(state, rooms):
    cost, outstanding_amphipods, cur_map = state
    for amphipod, amphipod_type, move_count in outstanding_amphipods:
        other_amphipods = outstanding_amphipods - {(amphipod, amphipod_type, move_count)}
        for dest, trip_cost in potential_moves(amphipod, amphipod_type, move_count, cur_map, rooms):
            new_map = cur_map.copy()
            new_map.update({amphipod: '.', dest: amphipod_type})
            new_cost = cost + trip_cost
            if dest in rooms[amphipod_type]:
                # this amphipod is no longer outstanding
                yield (new_cost, other_amphipods, new_map)
            else:
                yield (new_cost, other_amphipods | {(dest, amphipod_type, move_count + 1)}, new_map)


def initial_amphipod_states(map_, initial_positions, rooms):
    initial_amphipod_states = set()
    for p in initial_positions:
        typ = map_[p]
        if p[0] == rooms[typ][0][0] and all(map_[r] == typ for r in rooms[typ] if r[1] > p[1]):
            continue
        else:
            initial_amphipod_states.add((p, typ, 0))

    return frozenset(initial_amphipod_states)


def min_score(map_):
    initial_positions = sorted(pos for pos, v in map_.items() if v in 'ABCD')
    rooms = {k: tuple(positions) for k, (x, positions) in
             zip('ABCD', itertools.groupby(initial_positions, key=lambda k: k[0]))}

    init_states = initial_amphipod_states(map_, initial_positions, rooms)

    distance_to = defaultdict(lambda: float('inf'))
    visited = set()
    entry_counter = itertools.count()
    queue = [(0, next(entry_counter), init_states, map_)]
    min_cost = None
    while queue:
        cur_cost, _, cur_outstanding, cur_map = heapq.heappop(queue)
        if not cur_outstanding:
            return cur_cost

        if cur_outstanding in visited:
            continue

        visited.add(cur_outstanding)

        for next_cost, next_outstanding, next_map in successor_states((cur_cost, cur_outstanding, cur_map), rooms):
            if next_cost < distance_to[next_outstanding]:
                distance_to[next_outstanding] = next_cost
                heapq.heappush(queue, (next_cost, next(entry_counter), next_outstanding, next_map))

    return min_cost


def parse_input(s: str) -> Dict[Tuple[int, int], str]:
    map_ = {}
    for y, line in enumerate(s.splitlines()):
        for x, c in enumerate(line):
            map_[(x, y)] = c
    return map_


def main():
    parser = argparse.ArgumentParser(description='Solution for day 23 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file, 'r', encoding='utf8') as f:
        map_ = parse_input(f.read())

    print(min_score(map_))


if __name__ == '__main__':
    main()
