import os
import hashlib

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

def leading_zeros(s):
    count = 0
    for c in s:
        if c == "0":
            count += 1
        else:
            return count

def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        s = f.read()
    num = 1
    while True:
        hash = hashlib.md5((s + str(num)).encode('ascii')).digest().hex()
        if leading_zeros(hash) == 5:
            print(num)
            return
        num += 1

def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        s = f.read()
    num = 1
    while True:
        hash = hashlib.md5((s + str(num)).encode('ascii')).digest().hex()
        if leading_zeros(hash) == 6:
            print(num)
            return
        num += 1
 
def main():
    part_1()
    part_2()

if __name__ == "__main__":
    main()