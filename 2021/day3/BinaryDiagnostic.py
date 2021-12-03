report = open("BinaryDiagnostic.txt").read().splitlines()

bits = {}

most_common_bit = ""
least_common_bit = ""

for i in range(0,len(report[0])):
  bits[str(i)] = { "1s": 0, "0s": 0, "most common": None, "least common": None }
  bit = bits[str(i)]
  for j in range(0,len(report)):
    if report[j][i] == "1":
      bit["1s"] += 1
    else:
      bit["0s"] += 1
    if bit["1s"] > bit["0s"]:
      bit["most common"] = 1
      bit["least common"] = 0
    else:
      bit["most common"] = 0
      bit["least common"] = 1
  most_common_bit += str(bit["most common"])
  least_common_bit += str(bit["least common"])

print(int(most_common_bit, 2) * int(least_common_bit, 2))

# part 2

def bit_freq(rep, tiebreaker, j):
  ones = 0
  zeros = 0
  most_common_bit = None
  least_common_bit = None
  for i in range(0,len(rep)):
    if rep[i][j] == "1":
      ones += 1
    else:
      zeros += 1
  if ones > zeros:
    most_common_bit = "1"
    least_common_bit = "0"
  elif zeros > ones:
    most_common_bit = "0"
    least_common_bit = "1"
  else:
    if tiebreaker == 1:
      return "1"
    else:
      return "0"
  if tiebreaker == 1:
    return most_common_bit
  else:
    return least_common_bit

def bit_criteria(report, tiebreaker):
  bit = bit_freq(report, tiebreaker, 0)
  filtered = list(filter(lambda diagnostic: (diagnostic[0] == bit), report))
  for i in range(1, len(filtered)):
    bit = bit_freq(filtered, tiebreaker, i)
    filtered = list(filter(lambda diagnostic: (diagnostic[i] == bit), filtered))
    if len(filtered) == 1:
      return filtered[0]
  return filtered[0]

print(int(bit_criteria(report, 1), 2) * int(bit_criteria(report, 0), 2))
