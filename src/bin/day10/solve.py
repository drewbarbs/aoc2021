import argparse
from enum import Enum
from functools import reduce

PAIRS = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}

INVERSE_PAIRS = {v:k for k, v in PAIRS.items()}

ERROR_SCORE = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

COMPLETION_SCORE = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}

def get_error_score(line):
    stack = []
    for c in line:
        if c in PAIRS:
            stack.append(c)
        elif stack and c == PAIRS[stack[-1]]:
            stack.pop()
        elif stack:
            #this was an invalid delimeter, add to score
            return ERROR_SCORE[c]

def part1(lines):
    total_score = 0
    incomplete_lines = []
    for l in lines:
        score = get_error_score(l)
        if score is None:
            incomplete_lines.append(l)
        else:
            total_score += score

    return total_score, incomplete_lines


def get_incomplete_score(line):
    stack = []
    for c in line:
        if c in PAIRS:
            stack.append(c)
        elif stack and c == PAIRS[stack[-1]]:
            stack.pop()
        else:
            assert False, "This is a corrupt line!"
    completion = ''.join(PAIRS[c] for c in reversed(stack))
    return reduce(lambda acc, c: 5*acc + COMPLETION_SCORE[c], completion, 0)


def part2(lines):
    scores = [get_incomplete_score(l) for l in lines]
    scores.sort()
    return scores[len(scores)//2]

def main():
    parser = argparse.ArgumentParser(description='Solution for day 9 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = [l for l in f.read().splitlines() if l.strip()]

    p1_score, incomplete = part1(lines)
    print(p1_score)

    print(part2(incomplete))

if __name__ == '__main__':
    main()
