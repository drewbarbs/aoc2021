import argparse
from collections import defaultdict, deque

def parse_input(lines):
    outgoing = defaultdict(set)
    for l in lines:
        a, b = l.split('-')
        outgoing[a].add(b)
        outgoing[b].add(a)

    return outgoing

def part1(graph):
    # dfs
    paths = 0
    stack = deque([('start', {'start'})])

    while stack:
        node, visited = stack.pop()
        for next_node in graph[node]:
            v = visited.copy()
            if next_node == 'end':
                paths += 1
                continue

            if next_node.islower() and next_node not in visited:
                v.add(next_node)
                stack.append((next_node, v))
            elif next_node.isupper():
                stack.append((next_node, v))

    return paths

def part2(graph):
    # dfs
    paths = 0
    stack = deque([('start', {'start'}, False)])

    while stack:
        node, visited, revisited_small = stack.pop()
        for next_node in graph[node]:
            v = visited.copy()
            if next_node == 'start':
                continue
            elif next_node == 'end':
                paths += 1
                continue

            have_visited = next_node in visited
            if next_node.islower() and (not have_visited or not revisited_small):
                v.add(next_node)
                stack.append((next_node, v, revisited_small or have_visited))
            elif next_node.isupper():
                stack.append((next_node, v, revisited_small))

    return paths

def main():
    parser = argparse.ArgumentParser(description='Solution for day 12 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file) as f:
        lines = [l for l in f.read().splitlines() if l.strip()]

    graph = parse_input(lines)
    print(part1(graph))
    print(part2(graph))



if __name__ == '__main__':
    main()
