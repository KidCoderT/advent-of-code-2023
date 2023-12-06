with open("input.txt", "r") as file:
    input = file.read()

time, duration = input.splitlines()
time = int("".join(time.split()[1:]))
duration = int("".join(duration.split()[1:]))

start = 1

for i in range(1, time):
    a = (time - i) * i
    if a > duration:
        start = i
        break

end = 1
for i in reversed(range(time)):
    a = (time - i) * i
    if a > duration:
        end = i
        break

print(end - start + 1)
