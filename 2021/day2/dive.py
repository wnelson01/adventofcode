# part 1
input = [i.split(" ") for i in open("input.txt").read().splitlines()]

course = {
  "up": 0,
  "down": 0,
  "forward": 0
}

for i in input:
  course[i[0]] += int(i[1])
print((course["down"] - course["up"]) * course["forward"])

# part 2

course = {
  "aim": 0,
  "x": 0,
  "y": 0
}

for i in input:
  if i[0] == "up":
    course["aim"] -= int(i[1])
  elif i[0] == "down":
    course["aim"] += int(i[1])
  elif i[0] == "forward":
    course["x"] += int(i[1])
    course["y"] += course["aim"] * int(i[1])
  
print(course["x"] * course["y"])

