import re
from collections import Counter
from itertools import groupby

points = Counter()
entries = []

with open('input.txt') as f:
    for line in f:
        a_id, pos_x, pos_y, width, height = (int(e) for e in re.findall('\d+', line))
        curr = [(a_id, x+pos_x, y+pos_y) for x in range(width) for y in range(height)]
        entries.extend(curr)
        points.update((tuple(point) for _, *point in curr))

entries.sort(key=lambda x: x[0])

entries = groupby(entries, key=lambda x: x[0])

unique_points = {point for point, count in points.items() if count == 1}

for entry in entries:
    if all(tuple(point) in unique_points for a_id, *point in entry[1]):
        print(entry[0])
