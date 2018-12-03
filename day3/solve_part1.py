import re
from collections import Counter

points = Counter()

with open('input.txt') as f:
    for line in f:
        a_id, pos_x, pos_y, width, height = (int(e) for e in re.findall('\d+', line))
        curr = [(x+pos_x, y+pos_y) for x in range(width) for y in range(height)]
        points.update(curr)

print(len([point for point, count in points.items() if count > 1]))
