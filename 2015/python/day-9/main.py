import os

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


class Node:
    def __init__(self, id: str):
        self.neighbors = []
        self.id = id
        self.visited = False

    def __eq__(self, other):
        self.id == other.id

    def __hash__(self):
        return hash(self.id)


def traveling_salesman(curr):
    if all([n[0].visited for n in curr.neighbors]):
        return 0
    curr.visited = True
    result = min([traveling_salesman(n[0]) + n[1] for n in curr.neighbors if not n[0].visited])
    curr.visited = False
    return result

def stupid_salesman(curr):
    if all([n[0].visited for n in curr.neighbors]):
        return 0
    curr.visited = True
    result = max([stupid_salesman(n[0]) + n[1] for n in curr.neighbors if not n[0].visited])
    curr.visited = False
    return result



def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]

    nodes = {}
    for line in lines:
        (cities, dist) = line.split(" = ")
        cities = cities.split(" to ")
        dist = int(dist)

        for city in cities:
            if city not in nodes:
                nodes[city] = Node(city)

        nodes[cities[0]].neighbors.append((nodes[cities[1]], dist))
        nodes[cities[1]].neighbors.append((nodes[cities[0]], dist))

    print("part 1:", min(list([traveling_salesman(node) for node in nodes.values()])))


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]

    nodes = {}
    for line in lines:
        (cities, dist) = line.split(" = ")
        cities = cities.split(" to ")
        dist = int(dist)

        for city in cities:
            if city not in nodes:
                nodes[city] = Node(city)

        nodes[cities[0]].neighbors.append((nodes[cities[1]], dist))
        nodes[cities[1]].neighbors.append((nodes[cities[0]], dist))

    print("part 2:", max(list([stupid_salesman(node) for node in nodes.values()])))

def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
