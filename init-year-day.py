import sys
import shutil
import os
from distutils.dir_util import copy_tree


def main():
    if len(sys.argv) < 2:
        print("please provide a year")
        return
    if len(sys.argv) < 3:
        print("please provide a day")
        return

    year = int(sys.argv[1])
    day = int(sys.argv[2])

    source = "./template"
    destination = f"./{year}/day-{day}"

    if not os.path.exists(destination):
        os.mkdir(destination)

    dir_is_empty = (len(os.listdir(destination)) == 0)
    if not dir_is_empty:
        print(f"{destination} is not empty")
        return
    
    copy_tree(source, destination)

if __name__ == "__main__":
    main()
