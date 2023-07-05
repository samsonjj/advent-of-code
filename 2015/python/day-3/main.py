import os

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

def part_1():
    with open(os.path.join(__location__, 'input.txt'), 'r') as f:
        s = f.read()

    houses = {}
    pos = (0, 0)
    houses[pos] = 1
    for c in s:
        if c == "^":
            delta = (0, 1)
        elif c == ">":
            delta = (1, 0)
        elif c == "<":
            delta = (-1, 0)
        elif c == "v":
            delta = (0, -1)

        pos = (pos[0] + delta[0], pos[1] + delta[1])

        if pos in houses:
            houses[pos] += 1
        else:
            houses[pos] = 1

    print("part 1:", len(houses))

def part_2():
    with open(os.path.join(__location__, 'input.txt'), 'r') as f:
        s = f.read()

    delta = (0, 0)
    santa_pos = (0, 0)
    robo_pos = (0, 0)
    houses = {}
    houses[santa_pos] = 2
    robo_turn = False

    for c in s:
        if c == "^":
            delta = (0, 1)
        elif c == ">":
            delta = (1, 0)
        elif c == "<":
            delta = (-1, 0)
        elif c == "v":
            delta = (0, -1)

        if robo_turn:
            robo_pos = (robo_pos[0] + delta[0], robo_pos[1] + delta[1])
            pos = robo_pos
        else:
            santa_pos = (santa_pos[0] + delta[0], santa_pos[1] + delta[1])
            pos = santa_pos
        robo_turn = not robo_turn

        if pos in houses:
            houses[pos] += 1
        else:
            houses[pos] = 1

    print("part 2:", len(houses))
 
def main():
    part_1()
    part_2()

if __name__ == "__main__":
    main()