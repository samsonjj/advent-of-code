import os
from itertools import permutations

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

def main():
    with open(os.path.join(__location__, 'input.txt'), 'r') as f:
        s = f.read()
    paper = 0
    ribbon = 0
    for line in s.splitlines():
        (l, w, h) = [int(x) for x in line.split('x')]
        sides = [2*l*w, 2*w*h, 2*l*h]
        paper += sum(sides) + min(sides) // 2

        vol = l*w*h
        edges = [l, w, h]
        edges.remove(max(edges))
        perim = 2 * (edges[0] + edges[1])
        ribbon += perim + vol

    print("part 1:", paper)
    print("part 2:", ribbon)


if __name__ == "__main__":
    main()