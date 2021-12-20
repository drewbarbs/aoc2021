import argparse
from typing import List, Tuple


def parse_input(s) -> Tuple[List[List[str]], int, str]:
    it = iter(s.splitlines())
    alg = next(it)

    assert len(alg) == 512
    if alg[0] == '#':
        # if we're going to flip the infinite 0 pixels to 1, we need
        # to flip them back to 0 to make this tractable
        assert alg[511] == '.'

    assert next(it) == ''

    img = [[c for c in line] for line in it]

    return img, 0, alg


def enlarge(img, fill, alg) -> Tuple[List[List[str]], int]:
    cur_N = len(img)
    cur_M = len(img[0])

    new_img = [[None] * (cur_M + 2) for _ in range(cur_N + 2)]
    def get_alg_idx(new_y, new_x):
        idx = 0
        for r in range(new_y - 1, new_y + 2):
            for c in range(new_x - 1, new_x + 2):
                img_r, img_c = r - 1, c - 1
                if 0 <= img_r < cur_N and 0 <= img_c < cur_M:
                    px_val = 1 if img[img_r][img_c] == '#' else 0
                else:
                    px_val = fill
                idx = idx << 1 | px_val
                assert 0 <= idx < 512

        return idx

    for r in range(len(new_img)):
        for c in range(len(new_img[0])):
            new_img[r][c] = alg[get_alg_idx(r, c)]

    fill = int(alg[511] == '#') if fill == 1 else int(alg[0] == '#')

    return new_img, fill


def print_img(img):
    print('\n'.join(''.join(r) for r in img))


def num_lit_after_m_iterations(img, fill, alg, m):
    new_img = img
    new_fill = fill
    for _ in range(m):
        new_img, new_fill = enlarge(new_img, new_fill, alg)

    return sum(sum(1 for c in row if c == '#') for row in new_img)


def main():
    parser = argparse.ArgumentParser(description='Solution for day 20 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()
    with open(args.input_file, 'r', encoding='utf8') as f:
        input_ = f.read()

    img, fill, alg = parse_input(input_)

    print('Part 1: {} pixels lit'.format(num_lit_after_m_iterations(img, fill, alg, 2)))
    print('Part 2: {} pixels lit'.format(num_lit_after_m_iterations(img, fill, alg, 50)))


if __name__ == '__main__':
    main()
