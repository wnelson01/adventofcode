report = [int(i) for i in open("input.txt").read().splitlines()]
increases = 0
for i in range(1, len(report)):
  if report[i] > report[i - 1]:
    increases += 1
print(increases)

increases = 0
for i in range(5, len(report) - len(report) % 3):
  if report[i - 5] + report[i - 4] + report[i - 3] < report[i - 2] + report[i - 1] + report[i]:
    increases += 1
print(increases)