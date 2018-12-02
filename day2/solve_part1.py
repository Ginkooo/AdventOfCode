from collections import Counter

ids = {
    'two': set(),
    'three': set(),
}

counts = ((Counter(line), line) for line in open('input.txt'))

for count, line in counts:
    for v in count.values():
        if v == 2:
            ids['two'].add(line)
        if v == 3:
            ids['three'].add(line)

print(len(ids['two']) * len(ids['three']))
