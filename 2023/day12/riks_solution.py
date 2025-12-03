from collections import defaultdict

def fit_combinations(slots, to_fit):
    output = 0
    cache = {(slots, to_fit): 1}

    iterations = 0
    while True:
        new_cache = defaultdict(int)

        print(f"Cache at iteration {iterations} {cache.items()}")
        for key, paths in cache.items():
            slots_left, to_fit = key
            print(f"At iteration {iterations}: {slots_left} {to_fit}, {paths}")
            if sum(to_fit) > len(slots_left):
                break
            if slots_left[0] != "#":
                print("hit1", slots_left[1:], to_fit, paths)
                new_cache[(slots_left[1:], to_fit)] += paths

            print(slots_left, slots_left[:to_fit[0]])
            if all(c in ["#", "?"] for c in slots_left[:to_fit[0]]):
                if len(to_fit) == 1:
                    # check if no '#' left after last fit
                    if all(c != "#" for c in slots_left[to_fit[0]:]):
                        output += paths

                 # check for '.' directly after fit
                elif slots_left[to_fit[0]] in [".", "?"]:
                    print("hit2", slots_left[to_fit[0] + 1:], to_fit[1:], paths)
                    # add + 1 index here for the '.' after match
                    new_cache[(slots_left[to_fit[0] + 1:], to_fit[1:])] += paths

        if not new_cache:
            break
        iterations += 1
        cache = new_cache
    return output

if __name__ == "__main__":
    with open("input") as file:
        lines = [line.rstrip() for line in file]
    data = [line.split() for line in lines]
    data = [(line[0], tuple(map(int, line[1].split(",")))) for line in data]
    a = fit_combinations(*data[0])
    print(a)
