def part_1():
    with open('input.txt', 'r') as f:
        lines = f.readlines()

        twos = 0
        threes = 0

        for line in lines:
            counts = {}
            for c in line:
                if c in counts:
                    counts[c] += 1
                else:
                    counts[c] = 1
            found_two = False
            found_three = False
            for (c, count) in counts.items():
                if count == 2 and not found_two:
                    twos += 1
                    found_two = True
                if count == 3 and not found_three:
                    threes += 1
                    found_three = True

        print(twos * threes)

def part_2():
    with open('input.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]

        for line1 in lines:
            for line2 in lines:
                if line1 == line2:
                    continue

                diff_count = 0
                for (c1, c2) in zip(line1, line2):
                    if c1 != c2:
                        diff_count += 1

                if diff_count == 1:
                    print(line1)
                    print(line2)
                    return

def main():
    part_1()
    part_2()
    
if __name__ == "__main__":
    main()