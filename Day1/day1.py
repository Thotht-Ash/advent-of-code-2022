import argparse

parser = argparse.ArgumentParser()
parser.add_argument("file")
args = parser.parse_args()

calories = 0
elflist = []

with open(args.file, 'r') as f:
  for line in f:
    if line.strip():
        calories += int(line.strip())
    else:
        elflist.append(calories)
        calories = 0

elflist.sort(reverse=True)

print("Highest: {}, Second: {}, Third: {}, Sum: {}".format(elflist[0], elflist[1], elflist[2], elflist[0] + elflist[1] + elflist[2]))

