import os

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))


def perform(s):
    count = 1
    i = 1
    last = s[0]
    result = ""
    while i < len(s):
        if last != s[i]:
            result = result + str(count)
            result = result + str(last)
            count = 0
        last = s[i]
        count += 1
        if i == len(s) - 1:
            result = result + str(count)
            result = result + str(last)
            count = 0
        i += 1
    return result


def main():
    s = '1113122113'
    for i in range(40):
        s = perform(s)
    print("part 1:", len(s))

    for i in range(10):
        s = perform(s)
    print("part 2:", len(s))


if __name__ == "__main__":
    main()

