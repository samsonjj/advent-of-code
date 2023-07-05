import os
import hashlib

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def non_overlapping_pair(s):
    found = set()
    last = ""
    for i in range(len(s)-1):
        curr = s[i:i+2]
        if curr in found and curr != last:
            return True
        found.add(curr)
        last = curr
    return False


def repeat_with_inbetween(s):
    return any([
        s[i] == s[i+2]
        and s[i] != s[i+1]
        for i in range(len(s)-2)
    ])


def nice1(s):
    vowels = "aeiou"
    forbidden = ["ab", "cd", "pq", "xy"]

    one = sum([c in vowels for c in s]) >= 3
    two = any([s[x] == s[x+1] for x in range(len(s)-1)])
    three = not any(s[x:x+2] in forbidden for x in range(len(s)-1))
    return one and two and three


def nice2(s):
    return non_overlapping_pair(s) and repeat_with_inbetween(s)


def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        words = [x.strip() for x in f.readlines()]
    print("part 1:", sum([nice1(word) for word in words]))


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        words = [x.strip() for x in f.readlines()]
    print("part 2:", sum([nice2(word) for word in words]))


def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
