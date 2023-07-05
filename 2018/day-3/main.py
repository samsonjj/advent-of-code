def main():
    s = ""
    with open('input.txt', 'r') as f:
        s = [x.strip() for x in f.readlines()]

    my_tuple = (1, 2, 3)
    hm = {}
    squareCount = 0
    # print(s)

    for x in s:
        # print(x)
        [left, top] = x.split(",")
        [top, width] = top.split(": ")
        [width, height] = width.split("x")
        e = left.split("@ ")
        left = e[1]
        # print(left)
        # print(top)
        # print(width)
        # print(height)
        for y in range(int(width)):
            for z in range(int(height)):
                a = int(left) + y
                b = int(top) + z
                if (a,b) in hm:
                    hm[(a,b)] = hm[(a,b)] + 1
                else:
                    hm[(a,b)] = 1
        # x[1] = x.split(":")
        # x[2] = x.split("x")
        # print(x)
    count = 0
    for (k, v) in hm.items():
        if v > 1:
            count = count + 1
    print(count)
    
if __name__ == '__main__':
    main()