import os
from itertools import permutations

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

class Ingredient:
    def __init__(self, id):
        self.id = id
        self.cap = 0
        self.dur = 0
        self.fla = 0
        self.tex = 0
        self.cal = 0

    def __str__(self):
        return self.id + ": " + str(self.cap) \
            + str(self.dur) \
            + str(self.fla) \
            + str(self.tex) \
            + str(self.cal)

def parse_ingredients(s: str):
    ingredients = {}
    for line in s.splitlines():
        tokens = line.split(" ")
        ingredient = Ingredient(tokens[0][:-1])
        ingredient.cap = int(tokens[2][:-1])
        ingredient.dur = int(tokens[4][:-1])
        ingredient.fla = int(tokens[6][:-1])
        ingredient.tex = int(tokens[8][:-1])
        ingredient.cal = int(tokens[10])
        ingredients[ingredient.id] = ingredient
    return ingredients

class IngredientCount:
    def __init__(self, id, count):
        self.id = id
        self.count = count
    def __str__(self):
        return self.id + ": " + str(self.count)

def calc_cookie(ingredient_map, ingredient_counts):
    cap = 0
    dur = 0
    fla = 0
    tex = 0
    cal = 0

    print([str(ic) for ic in ingredient_counts])

    for ic in ingredient_counts:
        cap += ingredient_map[ic.id].cap * ic.count
        dur += ingredient_map[ic.id].dur * ic.count
        fla += ingredient_map[ic.id].fla * ic.count
        tex += ingredient_map[ic.id].tex * ic.count
        cal += ingredient_map[ic.id].cal * ic.count

        print(str(ingredient_map[ic.id]))
    print(cap, dur, fla, tex, cal)

    cap = max(cap, 0)
    dur = max(dur, 0)
    fla = max(fla, 0)
    tex = max(tex, 0)
    cal = max(cal, 0)
    # print(cap, dur, fla, tex, cal)

    result = cap * dur * fla * tex * cal
    if result > 0:
        print('cleannn')
        print(result)
    return result

    

def part_1():
    with open(os.path.join(__location__, "input.txt")) as f:
        s = f.read()
    ingredients = parse_ingredients(s)
    print([str(i) for i in ingredients.values()])
    max_val = 0
    for cs in counts():
        print("counts", [c for c in cs])
        ingredient_counts = [IngredientCount(list(ingredients.keys())[i], c) for i, c in enumerate(cs)]
        val = calc_cookie(ingredients, ingredient_counts)
        # print("val", val)
        if val > 0:
            print('nice', val)
        max_val = max(max_val, val)

    print("part 1:", max_val)

def counts():
    for i in range(100):
        #for j in range(100):
            # for k in range(100):
            #    if i + j + k <= 100:
            #        yield [i, j, k, 100 - i - j - k]
            if i <= 100:
                yield [i, 100 - i]
    


def part_2():
    with open(os.path.join(__location__, "input.txt")) as f:
        lines = [x.strip() for x in f.readlines()]
    print("part 2:", 2)


def main():
    part_1()
    part_2()


if __name__ == "__main__":
    main()
