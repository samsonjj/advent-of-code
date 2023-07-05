import os
import re
import json

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        s = f.read()

    nums = [int(x) for x in re.findall("-?[0-9]+", s)]
    print("part 1:", sum(nums))

def iterate(o):
    if type(o) is dict:
        return iterate_dict(o)
    if type(o) is list:
        return iterate_list(o)
    if type(o) is int:
        return o
    return 0

def iterate_dict(d):
    result = 0
    for val in d.values():
        if val == "red":
            return 0
        result += iterate(val)
    return result

def iterate_list(l):
    return sum([iterate(x) for x in l])


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        s = f.read()

    data = json.loads(s)

    print("part 2:", iterate(data))


def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
