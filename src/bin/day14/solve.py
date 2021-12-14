import argparse
import collections
import itertools
from typing import Dict, Tuple
from functools import lru_cache


def parse_input(s: str) -> Tuple[str, Dict[str, str]]:
    lines = s.splitlines()
    template = lines[0].strip()

    rules = {}
    for r in lines[2:]:
        pair, _, insert = r.partition(' -> ')
        rules[(pair[0], pair[1])] = insert

    return template, rules


# From python docs, for python < 3.10
def pairwise(iterable):
    # pairwise('ABCDEFG') --> AB BC CD DE EF FG
    a, b = itertools.tee(iterable)
    next(b, None)
    return zip(a, b)


def step(template: str, rules: Dict[str, str]) -> str:
    insertions = [None] * len(template)
    insertions[-1] = ''
    for i, pair in enumerate(pairwise(template)):
        insertions[i] = rules[pair]
    return ''.join(itertools.chain.from_iterable(zip(template, insertions)))


def answer_from_counter(counter: collections.Counter) -> int:
    [(_, most_common_count)] = counter.most_common(1)
    _, least_common_count = counter.most_common()[-1]
    return most_common_count - least_common_count


def solve_naive(template: str, rules: Dict[str, str], steps: int) -> int:
    for i in range(steps):
        template = step(template, rules)

    return answer_from_counter(collections.Counter(template))


def main():
    parser = argparse.ArgumentParser(description='Solution for day 14 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        template, rules = parse_input(f.read())

    print(solve_naive(template, rules, 10))

    @lru_cache(maxsize=None)
    def do_expand(template, n):
        if n == 1:
            expanded = step(template, rules)
            return collections.Counter(expanded)

        counter = collections.Counter()
        template = step(template, rules)
        for p in pairwise(template):
            counter += do_expand(''.join(p), n - 1)
            # we'll double-count the second element if we don't
            # subtract here
            counter.subtract(p[1])
        # except the last element
        counter.update(template[-1])

        return counter

    result = do_expand(template, 40)
    print(answer_from_counter(result))


if __name__ == '__main__':
    main()
