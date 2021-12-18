import argparse
import math
import re

def parse(s):
    m = re.search('target area: x=([-0-9]+)\\.\\.([-0-9]+), y=([-0-9]+)\\.\\.([-0-9]+)',
                 s)
    return int(m.group(1)), int(m.group(2)), int(m.group(3)), int(m.group(4))


def solve_for_v_min(t_x_min):
    # min x velocity is such that we reach the left edge of target
    # area just as our velocity gets to 0: v + (v-1) + ... + 0 = t_x_min.
    # Solve using summation identity (1 + .. + v == v*(v+1)/2)
    a = 1
    b = 1
    c = -2*t_x_min
    possibilities = [((-1*b) + disc)/(2*a) for disc in (-1 * math.sqrt(b**2 - 4*a*c), math.sqrt(b**2 - 4*a*c))]
    bound, = list(filter(lambda v: v>= 0, possibilities))
    return int(math.floor(bound))


def solve(t_x_min, t_x_max, t_y_min, t_y_max):
    v_x_max = t_x_max
    v_x_min = solve_for_v_min(t_x_min)
    v_y_max = max(abs(t_y_min), abs(t_y_max))
    v_y_min = -1 * v_y_max
    total_max_y_pos = 0
    feasible = set()
    for initial_vx in range(v_x_min, v_x_max+1):
        for initial_vy in range(v_y_min, v_y_max):
            vx = initial_vx
            vy = initial_vy
            max_y_pos = 0
            pos_x = 0
            pos_y = 0
            while (pos_x < t_x_min and vx) or pos_y > t_y_max:
                pos_x += vx
                pos_y += vy
                max_y_pos = max(pos_y, max_y_pos)
                if vx != 0:
                    vx += math.copysign(1, 0 - vx)
                vy -= 1

            if t_x_min <= pos_x <= t_x_max and t_y_min <= pos_y <= t_y_max:
                # print('feasible!', initial_vx, initial_vy)
                total_max_y_pos = max(total_max_y_pos, max_y_pos)
                feasible.add((initial_vx, initial_vy))
    return total_max_y_pos, len(feasible)


def main():
    parser = argparse.ArgumentParser(description='Solution for day 17 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file, 'r') as f:
        input_ = f.read()

    t_x_min, t_x_max, t_y_min, t_y_max = parse(input_.strip())

    # largest x velocity we can have is getting to right edge in one
    # shot
    print(solve(t_x_min, t_x_max, t_y_min, t_y_max))


if __name__ == '__main__':
    main()
