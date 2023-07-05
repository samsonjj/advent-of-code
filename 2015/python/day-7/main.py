import os
import hashlib

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

class OrderError(Exception):
    pass

def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    lines = list(lines)
    m = {}

    while len(lines) > 0:
        unprocessed_lines = []
        for line in lines:

            def val(token):
                try:
                    return int(token)
                except:
                    if token in m:
                        return m[token]
                    else:
                        unprocessed_lines.append(line)
                        raise OrderError('the token is not an int, and was not found in the map')

            try:
                (input, out) = line.split(' -> ')
                input = input.split(' ')
                if len(input) == 1:
                    m[out] = val(input[0])
                elif len(input) == 2:
                    m[out] = ~val(input[1])
                elif len(input) == 3:
                    op = input[1]
                    if op == "AND":
                        m[out] = val(input[0]) & val(input[2])
                    elif op == "OR":
                        m[out] = val(input[0]) | val(input[2])
                    elif op == "LSHIFT":
                        m[out] = val(input[0]) << val(input[2])
                    elif op == "RSHIFT":
                        m[out] = val(input[0]) >> val(input[2])
                    else:
                        raise Exception('uh oh 1')
                else:
                    raise Exception('uh oh 2')
            except:
                continue
        lines = unprocessed_lines

    print(m['a'])
    return m['a']
        

def part_2(b_val):
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    lines = list(lines)
    m = {}

    while len(lines) > 0:
        unprocessed_lines = []
        for line in lines:

            def val(token):
                try:
                    return int(token)
                except:
                    if token == 'b':
                        return b_val
                    if token in m:
                        return m[token]
                    else:
                        unprocessed_lines.append(line)
                        raise OrderError('the token is not an int, and was not found in the map')

            try:
                (input, out) = line.split(' -> ')
                input = input.split(' ')
                if len(input) == 1:
                    m[out] = val(input[0])
                elif len(input) == 2:
                    m[out] = ~val(input[1])
                elif len(input) == 3:
                    op = input[1]
                    if op == "AND":
                        m[out] = val(input[0]) & val(input[2])
                    elif op == "OR":
                        m[out] = val(input[0]) | val(input[2])
                    elif op == "LSHIFT":
                        m[out] = val(input[0]) << val(input[2])
                    elif op == "RSHIFT":
                        m[out] = val(input[0]) >> val(input[2])
                    else:
                        raise Exception('uh oh 1')
                else:
                    raise Exception('uh oh 2')
            except:
                continue
        lines = unprocessed_lines

    print(m['a'])


def main():
    result = part_1()
    part_2(result)


if __name__ == "__main__":
    main()
