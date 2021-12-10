import argparse
from enum import Enum
from functools import reduce
from typing import Optional

PAIRS = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}

ERROR_SCORE = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

COMPLETION_SCORE = {
    '(': 1,
    '[': 2,
    '{': 3,
    '<': 4,
}

class ScoreType(Enum):
    ERROR = 1
    COMPLETION = 2

def get_score(line, score_type: ScoreType) -> Optional[int]:
    stack = []
    for c in line:
        if c in PAIRS:
            stack.append(c)
        elif stack and c == PAIRS[stack[-1]]:
            stack.pop()
        elif score_type == ScoreType.ERROR:
            #this was an invalid delimeter, add to score
            return ERROR_SCORE[c]
        else:
            return None

    if score_type == ScoreType.COMPLETION:
        return reduce(lambda acc, c: 5*acc + COMPLETION_SCORE[c], reversed(stack), 0)

def part1(lines):
    scores = [get_score(l, ScoreType.ERROR) for l in lines]
    return sum(s for s in scores if s is not None)

def part2(lines):
    scores = [get_score(l, ScoreType.COMPLETION) for l in lines]
    scores = [s for s in scores if s is not None]
    scores.sort()
    return scores[len(scores)//2]

def main():
    parser = argparse.ArgumentParser(description='Solution for day 10 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = [l for l in f.read().splitlines() if l.strip()]

    print(part1(lines))
    print(part2(lines))

if __name__ == '__main__':
    main()
