import os
import hashlib
import time

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]

    lights = [[False for x in range(1000)] for y in range(1000)]

    for line in lines:
        parts = line.split(" ")
        l = len(parts)

        tl = [int(x) for x in parts[l-3].split(',')]
        br = [int(x) for x in parts[l-1].split(',')]
        command = " ".join(parts[:l-3])

        if command == "turn on":
            def f(x, y):
                lights[y][x] = True
        elif command == "turn off":
            def f(x, y):
                lights[y][x] = False
        elif command == "toggle":
            def f(x, y):
                lights[y][x] = not lights[y][x]
        else:
            raise Exception('oh no!')

        for y in range(tl[1],br[1]+1):
            for x in range(tl[0],br[0]+1):
                f(x, y)

    print("part 1:", sum([sum(row) for row in lights]))


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]

    lights = [[0 for x in range(1000)] for y in range(1000)]
    print('hello')

    for line in lines:
        parts = line.split(" ")
        l = len(parts)

        tl = [int(x) for x in parts[l-3].split(',')]
        br = [int(x) for x in parts[l-1].split(',')]
        command = " ".join(parts[:l-3])

        if command == "turn on":
            def f(x, y):
                lights[y][x] += 1
        elif command == "turn off":
            def f(x, y):
                lights[y][x] = max(0, lights[y][x] - 1)
        elif command == "toggle":
            def f(x, y):
                lights[y][x] += 2
        else:
            raise Exception('oh no!')

        for y in range(tl[1],br[1]+1):
            for x in range(tl[0],br[0]+1):
                f(x, y)

    print("part 2:", sum([sum(row) for row in lights]))


def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
