import os

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

def is_secure(p):
    increasing_count = 0
    last_ord = 0
    has_inc_seq = False
    contains_iol = False
    last = '0'
    pair = '00'
    pair_count = 0
    for c in p:
        if ord(c) == last_ord + 1:
            increasing_count += 1
        else:
            increasing_count = 1

        if increasing_count == 3:
            has_inc_seq = True

        if c in 'iol':
            contains_iol

        if c == last and 2 * c != pair:
            pair = 2 * c
            pair_count += 1
        last = c
        last_ord = ord(c)

    return has_inc_seq and not contains_iol and pair_count >= 2

def increment(p):
    i = len(p) - 1
    while i >= 0:
        s = list(p)
        s[i] = chr(ord(p[i]) + 1)
        p = "".join(s)
        if ord(p[i]) > 122:
            s = list(p)
            s[i] = 'a'
            p = "".join(s)
            i -= 1
        else:
            return p


def part_1(p):
    while not is_secure(p):
        p = increment(p)
    print("part 1:", p)
    return p


def part_2(p):
    while not is_secure(p):
        p = increment(p)
    print("part 2:", p)
    return p


def main():
    part_1('hepxcrrq')
    part_2('hepxxzaa')


if __name__ == "__main__":
    main()
