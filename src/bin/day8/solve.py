import argparse
from collections import defaultdict

SEGMENT_MAP = {
    '0': 'abcefg',
    '1': 'cf',
    '2': 'acdeg',
    '3': 'acdfg',
    '4': 'bcdf',
    '5': 'abdfg',
    '6': 'abdefg',
    '7': 'acf',
    '8': 'abcdefg',
    '9': 'abcdfg',
}

INVERSE_SEGMENT_MAP = {v:k for k, v in SEGMENT_MAP.items()}

UNIQUE_COUNTS = {2, 3, 4, 7}


def part1(lines):
    unique_count = 0
    for l in lines:
        if not l.strip():
            continue
        notes, output = l.split('|')
        for digit in output.split(' '):
            if len(digit.strip()) in UNIQUE_COUNTS:
                unique_count += 1

    return unique_count

def decode_mapping(notes):
    observed = [n.strip() for n in notes.split(' ') if n.strip()]

    # initially, any segment could map to any segment
    mapping = {}
    # use the digits with unique counts to get initial constraints
    for_count = defaultdict(list)
    for n in observed:
        for_count[len(n)].append(n)

    two_count_set = set(for_count[2][0])
    three_count_set = set(for_count[3][0])
    cf_possibilities = two_count_set
    # 'a' is uniquely determined by 1 and 7
    mapping['a'], = three_count_set.difference(cf_possibilities)
    # 3 and 4 uniquely determine what "d" is mapped to
    four_count_set = set(for_count[4][0])
    bd_possibilities = four_count_set.difference(cf_possibilities)
    three_digit, = [d for d in for_count[5] if not cf_possibilities.difference(d)]

    adg_possibilities = set(three_digit).difference(cf_possibilities)
    mapping['d'], = adg_possibilities.intersection(bd_possibilities)
    bd_possibilities.remove(mapping['d'])
    mapping['b'], = bd_possibilities

    # since we have b, d uniquely determined, we can figure out which
    # of the five_counts corresponds to the digit 5
    bd_set = {mapping['b'], mapping['d']}
    five_digit, = [digit for digit in for_count[5] if not bd_set.difference(set(digit))]
    mapping['f'], = set(five_digit).intersection(two_count_set)
    mapping['c'], = cf_possibilities.difference({mapping['f']})
    mapping['g'], = set(five_digit).difference({mapping[c] for c in 'abdf'})
    mapping['e'], = set('abcdefg').difference(mapping.values())

    return mapping

def get_digit(inverse_mapping, n):
    canonical = ''.join(sorted(inverse_mapping[c] for c in n))
    return INVERSE_SEGMENT_MAP[canonical]

def part2(lines):
    answer = 0
    for l in lines:
        if not l.strip():
            continue
        notes, output = l.split('|')
        output_numbers = [n.strip() for n in output.split(' ') if n.strip()]
        mapping = decode_mapping(notes)
        inverse_mapping = {v:k for k, v in mapping.items()}
        number = ''.join(get_digit(inverse_mapping, n) for n in output_numbers)
        answer += int(number)

    return answer


def main():
    parser = argparse.ArgumentParser(description='Solution for day 8 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = f.read().splitlines()

    print('Part 1:', part1(lines))
    print('Part 2:', part2(lines))


if __name__ == '__main__':
    main()
