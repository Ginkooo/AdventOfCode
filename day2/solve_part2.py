from itertools import combinations

with open('input.txt') as f:
    for left, right in combinations(f.readlines(), 2):
        comm_letters = [cl for cl, cr in zip(left, right) if cl == cr]
        if len(comm_letters) == len(left) - 1:
            print(''.join(comm_letters))
