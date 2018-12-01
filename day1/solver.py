from itertools import cycle

freq = 0
freqs = {0}

with open('input.txt') as f:
    for line in cycle(f):
        freq += eval(line)
        if freq in freqs:
            print(freq)
            exit()
        freqs.add(freq)
