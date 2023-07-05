import os

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    result = 0
    for line in lines:
        code_chars = len(line)
        # remove start and end quotes
        line = line[1:-1]
        mem_chars = 0
        i = 0
        while i < len(line):
            mem_chars += 1
            if line[i] == "\\":
                i += 1
                if line[i] == "x":
                    i += 2
            i += 1
        result += code_chars - mem_chars

    print("part 1:", result)


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    result = 0
    for line in lines:
        mem_chars = len(line)
        # set to 2, for start and end quotes
        code_chars = 2
        i = 0
        while i < len(line):
            code_chars += 1
            if line[i] == "\\":
                code_chars += 1
            if line[i] == "\"":
                code_chars += 1
            i += 1
        result += code_chars - mem_chars


    print("part 2:", result)


def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
