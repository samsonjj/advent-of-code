import os

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    print("part 1:", 1)


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    print("part 2:", 2)


def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
