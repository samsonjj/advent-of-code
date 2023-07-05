def main():
    with open('input.txt', 'r') as f:
        s = f.read()
    floor = 0
    found = False
    for (i, c) in enumerate(s):
        if c == '(':
            floor += 1
        elif c == ')':
            floor -= 1
        else:
            print('error!')
            exit()
        if not found and floor < 0:
                print('negative at', i+1)
                found = True

    print(floor)


if __name__ == "__main__":
    main()