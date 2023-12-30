from collections import Counter

with open("input.txt", "r") as f:
    input = f.readlines()

deal_ranks = []

for deal in input:
    cards, bid = deal.split()
    counter = Counter(list(cards))
    rank = None

    first, *rest = counter.most_common()

    if first[1] in [5, 4]:
        rank = first[1] + 1

    elif first[1] == 3 and len(rest) == 1:
        rank = 4

    elif first[1] == 3 and len(rest) == 2:
        rank = 3

    elif first[1] == 2 and rest[0][1] == 2:
        rank = 2

    elif first[1] == 2 and rest[0][1] == 1:
        rank = 1

    else:
        rank = 0

    deal_ranks.append(rank)

sorted_list = sorted(
    list(
        zip(
            deal_ranks,
            map(lambda x: x.split()[0], input),
            map(lambda x: int(x.split()[1]), input),
        )
    ),
    key=lambda x: x[0],
    reverse=True,
)

segregated = [[val for val in sorted_list if val[0] == i] for i in reversed(range(7))]


def compare(deck1, deck2):
    # check if first > second

    mapping = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"]

    for d1, d2 in zip(deck1, deck2):
        if d1 == d2:
            continue

        return mapping.index(d1) < mapping.index(d2)


final_output = []

for segregation in segregated:
    if len(segregation) == 0:
        continue

    array = segregation.copy()
    n = len(segregation)

    for i in range(n):
        # Create a flag that will allow the function to
        # terminate early if there's nothing left to sort
        already_sorted = True

        # Start looking at each item of the list one by one,
        # comparing it with its adjacent value. With each
        # iteration, the portion of the array that you look at
        # shrinks because the remaining items have already been
        # sorted.
        for j in range(n - i - 1):
            if compare(array[j][1], array[j + 1][1]):
                # If the item you're looking at is greater than its
                # adjacent value, then swap them
                array[j], array[j + 1] = array[j + 1], array[j]

                # Since you had to swap two elements,
                # set the `already_sorted` flag to `False` so the
                # algorithm doesn't finish prematurely
                already_sorted = False

        # If there were no swaps during the last iteration,
        # the array is already sorted, and you can terminate
        if already_sorted:
            break

    final_output.extend(reversed(array))


score = 0
for i, value in enumerate(final_output):
    score += (len(final_output) - i) * value[2]

print(score)
