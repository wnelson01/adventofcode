report = [int(i) for i in open("input.txt").read().splitlines()]
increases = 0
for i in range(1, len(report)):
  if report[i] > report[i - 1]:
    increases += 1
print(increases)

increases = 0
# report = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
for i in range(3, len(report)):
  if report[i] > report[i - 3]:
    increases += 1
print(increases)