import argparse
import itertools
from functools import cache

from frozendict import frozendict

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


def potential_moves(start_pos, move_count, cur_map, rooms) -> list[((int, int), int)]:
    # figure out spots we have paths to using bfs
    amphipod_type = cur_map[start_pos]
    assert amphipod_type in 'ABCD'
    doorways = [(spots[0][0], spots[0][1] - 1) for spots in rooms.values()]
    other_room_spaces = set(s for typ, spaces in rooms.items() if typ != amphipod_type for s in spaces)

    # amphipod can only enter its room if no other type of amphipod is in that room,
    # will go to the furthest-in space
    if any(cur_map[s] in 'ABCD' and cur_map[s] != amphipod_type for s in rooms[amphipod_type]):
        target_spot = None
    else:
        available_spaces = [space for space in rooms[amphipod_type] if cur_map[space] == '.']
        target_spot = max(available_spaces) if available_spaces else None

    if move_count == 1 and target_spot is None:
        return []

    options = []
    visited = {start_pos}
    frontier = [(start_pos, 0)]
    while frontier:
        (x, y), cost = frontier.pop(0)
        for next_pos in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
            if next_pos in visited or cur_map[next_pos] != '.':
                continue

            frontier.append((next_pos, cost + ENERGY_PER_STEP[amphipod_type]))
            visited.add(next_pos)

            if next_pos in other_room_spaces:
                continue

            # can we actually stop here?
            if next_pos == target_spot:
                options.append(frontier[-1])
            elif move_count == 0 and next_pos not in doorways:
                options.append(frontier[-1])

    return options


def successor_states(state, rooms):
    successors = []
    outstanding_amphipods, cost, cur_map = state
    for amphipod, move_count in outstanding_amphipods:
        options = potential_moves(amphipod, move_count, cur_map, rooms)
        if not options:
            # deadlock
            continue

        amphipod_type = cur_map[amphipod]
        other_amphipods = outstanding_amphipods - {(amphipod, move_count)}
        for dest, trip_cost in options:
            new_map = cur_map | {amphipod: '.', dest: amphipod_type}
            if dest in rooms[amphipod_type]:
                # this amphipod is no longer outstanding
                successors.append((other_amphipods, cost + trip_cost, new_map))
            else:
                successors.append((other_amphipods | {(dest, move_count + 1)}, cost + trip_cost, new_map))

    return successors


@cache
def min_score_for_state(state, rooms):
    min_score = None
    for s in successor_states(state, rooms):
        if not s[0]:
            min_score = min(min_score, s[1]) if min_score is not None else s[1]
        elif score := min_score_for_state(s, rooms):
            min_score = min(min_score, score) if min_score is not None else score

    return min_score


def min_score(map_):
    initial_positions = sorted(pos for pos, v in map_.items() if v in 'ABCD')
    rooms = {k: tuple(positions) for k, (x, positions) in
             zip('ABCD', itertools.groupby(initial_positions, key=lambda k: k[0]))}
    rooms = frozendict(rooms)

    finish_states = []
    # run all possible states (amphipod positions/n moves, cost, map)
    initial_amphipod_states = frozenset((p, 0) for p in initial_positions if p != rooms[map_[p]][1])
    states = {(initial_amphipod_states, 0, map_)}

    seen_states = set()
    min_cost = None
    while states:
        state = states.pop()
        for succ in successor_states(state, rooms):
            outstanding = succ[0]
            if not outstanding:
                finish_states.append(succ)
                min_cost = min(succ[1], min_cost or float('inf'))
            elif (min_cost is None or succ[1] < min_cost) and succ not in seen_states:
                states.add(succ)
                seen_states.add(succ)

    return min_cost


def parse_input(s: str) -> dict[(int, int), str]:
    map_ = {}
    for y, line in enumerate(s.splitlines()):
        for x, c in enumerate(line):
            map_[(x, y)] = c
    return frozendict(map_)


def main():
    parser = argparse.ArgumentParser(description='Solution for day 23 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file, 'r', encoding='utf8') as f:
        map_ = parse_input(f.read())

    print(min_score(map_))


if __name__ == '__main__':
    main()
