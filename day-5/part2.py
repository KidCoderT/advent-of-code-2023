with open("./input.txt", "r", encoding="utf-8") as file:
    input = file.read()

seeds, *maps = input.split("\n\n")

last_map = {}
new_map = {}

seed_ranges = []  # [seed for seed in map(int, seeds.split(": ")[-1].split())]
seed_input = [int(val) for val in seeds.split(": ")[-1].split()]

for index, seed in enumerate(seed_input[::2]):
    seed_ranges.append((seed, seed + seed_input[((index * 2) + 1)]))


print(seed_ranges)

seed_values = [val for val in seed_ranges]

for mapping_set in maps:
    _, *values = mapping_set.splitlines()

    seed_maps = []

    map_range = {}
    for value in values:
        dest, source, _range = map(int, value.split())
        map_range[(source, source + _range)] = dest

    print(len(map_range))

    for st, ed in seed_ranges:
        for _range, dest in map_range.items():
            if _range[0] <= st and ed <= _range[1]:
                seed_maps.append((dest + (st - _range[0]), dest + (ed - _range[0])))
            elif st < _range[0] and _range[0] <= ed <= _range[1]:
                seed_maps.append(
                    (dest + (ed - _range[0]), dest + (_range[1] - _range[0]))
                )
            elif _range[0] <= st <= _range[1] and _range[1] < ed:
                seed_maps.append((dest, dest + (st - _range[0])))

    seed_ranges = seed_maps

print(min([range[0] for range in seed_ranges]))
