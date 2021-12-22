import argparse
import itertools

from collections import defaultdict


def part1(p1_start, p2_start):
    dice = itertools.cycle(range(1, 101))

    # player 1 starts first
    player_states = [(0, p1_start), (0, p2_start)]

    n_rolls = 0
    while max(s[0] for s in player_states) < 1000:
        for pid in range(len(player_states)):
            cur_score, cur_pos = player_states[pid]

            steps = sum(next(dice) for _ in range(3))
            n_rolls += 3
            next_pos = ((cur_pos + steps - 1) % 10) + 1
            next_score = cur_score + next_pos

            player_states[pid] = (next_score, next_pos)
            if next_score >= 1000:
                break

    return n_rolls * min(s[0] for s in player_states)


def part2(p1_start, p2_start):
    # map current position/score to count
    player_states = {(p1_start, 0, p2_start, 0): 1}
    n_wins = [0, 0]

    possible_roll_seqs = defaultdict(int)
    for rolls in itertools.product(range(1, 4), range(1, 4), range(1, 4)):
        possible_roll_seqs[sum(rolls)] += 1

    while any(player_states):
        new_player_states = defaultdict(int)
        for (p1_pos, p1_score, p2_pos, p2_score), count in player_states.items():
            for p1_steps, p1_steps_possibilities in possible_roll_seqs.items():
                next_p1_pos = ((p1_pos + p1_steps - 1) % 10) + 1
                next_p1_score = p1_score + next_p1_pos

                if next_p1_score >= 21:
                    n_wins[0] += p1_steps_possibilities * count
                else:
                    for p2_steps, p2_steps_possibilities in possible_roll_seqs.items():
                        next_p2_pos = ((p2_pos + p2_steps - 1) % 10) + 1
                        next_p2_score = p2_score + next_p2_pos

                        if next_p2_score >= 21:
                            n_wins[1] += p1_steps_possibilities * p2_steps_possibilities * count
                        else:
                            new_state = (next_p1_pos, next_p1_score,
                                         next_p2_pos, next_p2_score)
                            new_player_states[new_state] += p1_steps_possibilities * p2_steps_possibilities * count
        player_states = new_player_states

    return max(n_wins)


def main():
    parser = argparse.ArgumentParser(description='Solution for day 21 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()
    with open(args.input_file, 'r', encoding='utf8') as f:
        input_ = f.read()

    p1_start, p2_start = [int(l.split(':')[1]) for l in input_.splitlines()]

    print(part1(p1_start, p2_start))
    print(part2(p1_start, p2_start))


if __name__ == '__main__':
    main()
