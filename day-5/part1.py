with open("./input.txt", "r", encoding="utf-8") as file:
    input = file.read()

seeds, *maps = input.split("\n\n")

last_map = {}
new_map = {}

seed_values = [seed for seed in map(int, seeds.split(": ")[-1].split())]

for mapping_set in maps:
    _, *values = mapping_set.splitlines()

    map_range = {}
    for value in values:
        dest, source, _range = map(int, value.split())
        map_range[(source, source + _range)] = dest

    for i in range(len(seed_values)):
        for _range, dest in map_range.items():
            if _range[0] <= seed_values[i] <= _range[1]:
                seed_values[i] = dest + (seed_values[i] - _range[0])
                break

print(min(seed_values))
