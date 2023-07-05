import os
from itertools import permutations

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]

    happiness = {}
    people = set()

    for line in lines:
        tokens = line.split(" ")
        sign = 1 if tokens[2] == "gain" else -1
        amount = int(tokens[3])
        p1 = tokens[0]
        p2 = tokens[-1][:-1]
        happiness[(p1, p2)] = sign * amount
        people.add(p1)
        people.add(p2)

    people = list(people)
    
    result = -10000000 
    for ordering in permutations(people):
        temp = 0
        for i in range(len(ordering)):
            pair1 = (ordering[i], ordering[(i+1) % len(ordering)])
            pair2 = (ordering[(i+1) % len(ordering)], ordering[i])
            temp += happiness[pair1]
            temp += happiness[pair2]
        if temp > result:
            result = temp


    print("part 1:", result)


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]

    happiness = {}
    people = set()

    for line in lines:
        tokens = line.split(" ")
        sign = 1 if tokens[2] == "gain" else -1
        amount = int(tokens[3])
        p1 = tokens[0]
        p2 = tokens[-1][:-1]
        happiness[(p1, p2)] = sign * amount
        people.add(p1)
        people.add(p2)
        

    for p in people:
        happiness[(p, 'me')] = 0
        happiness[('me', p)] = 0
    people.add('me')

    people = list(people)
    
    result = -10000000 
    for ordering in permutations(people):
        temp = 0
        for i in range(len(ordering)):
            pair1 = (ordering[i], ordering[(i+1) % len(ordering)])
            pair2 = (ordering[(i+1) % len(ordering)], ordering[i])
            temp += happiness[pair1]
            temp += happiness[pair2]
        if temp > result:
            result = temp

    print("part 2:", result)




def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
