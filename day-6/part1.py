with open("input.txt", "r") as file:
    input = file.read()

time, duration = input.splitlines()
ans = 1
for time, duration in zip(map(int, time.split()[1:]), map(int, duration.split()[1:])):
    speeds = []
    for i in range(1, time):
        a = (time - i) * i
        if a > duration:
            speeds.append(a)
    ans *= len(speeds)

print(ans)
