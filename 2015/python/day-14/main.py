import os

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    
    max_dist = 0
    max_deer = ""
    for line in lines:
        tokens = line.split(" ")
        speed = int(tokens[3])
        fly_time = int(tokens[6])
        rest_time = int(tokens[13])
        print(speed, fly_time, rest_time)

        dist = 0
        flying = True
        time_left = fly_time
        for i in range(2503):
            if time_left == 0:
                flying = not flying
                if flying:
                    time_left = fly_time
                else:
                    time_left = rest_time
            if flying:
                dist += speed
            time_left -= 1
        print(dist)
        if dist > max_dist:
            max_dist = dist
            max_deer = tokens[0]

    print("part 1:", max_dist)


class Deer:
    def __init__(self, speed, fly, rest):
        self.speed = speed
        self.fly = fly
        self.rest = rest
        self.flying = True
        self.time_left = fly
        self.dist = 0
        self.score = 0

    def turn(self):
        if self.time_left == 0:
            self.flying = not self.flying
            if self.flying:
                self.time_left = self.fly
            else:
                self.time_left = self.rest
        if self.flying:
            self.dist += self.speed
        self.time_left -= 1



def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]

    deer = [
        Deer(int(tokens[3]), int(tokens[6]), int(tokens[13]))
        for tokens in [line.split(' ') for line in lines]
    ]

    print(len(deer))
    print([d.speed for d in deer])

    for i in range(2503):
        for d in deer:
            d.turn()
        max_deer = max(deer, key=lambda d: d.dist)
        max_deer.score += 1

    print("part 2:", max(deer, key=lambda d: d.score).score)

    




def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
