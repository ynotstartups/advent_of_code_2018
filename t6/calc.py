import string
from collections import Counter

import numpy as np


def m_distance(x, y):
    return abs(x[0] - y[0]) + abs(x[1] - y[1])

assert m_distance([0, 0], [0, 0]) == 0
assert m_distance([0, 0], [0, 1]) == 1
assert m_distance([0, 0], [1, 0]) == 1
assert m_distance([0, 0], [1, 1]) == 2
assert m_distance([2, 2], [1, 1]) == 2

def min_distance_to_all(coordinate, other_coordinates):

    m_distances = [m_distance(coordinate, i) for i in other_coordinates]

    # min_m_distance = min(m_distances)

    # if m_distances.count(min_m_distance) == 1:
    #     return m_distances.index(min_m_distance)
    # else:
    #     return -1

    if sum(m_distances) < 10000:
        return 1
    else:
        return 0


    
data = [
    [1, 1],
    [1, 6],
    [8, 3],
    [3, 4],
    [5, 5],
    [8, 9]
]

data =[]

with open('./input.txt', 'r') as f:
    for line in f:
        line = [int(i) for i in line.split(',')]
        data.append(line)

max_horizontal = max([i[0] for i in data]) + 2
max_vertial = max([i[1] for i in data]) + 1

results = []
infinite_groups = set()
for x in range(max_horizontal):
    for y in range(max_vertial):
        min_group = min_distance_to_all([x, y], data)
        results.append(min_group)

        # if x == 0 or y == 0 or x == max_horizontal-1 or y ==max_vertial-1:
        #     if min_group != -1:
        #         infinite_groups.add(min_group)

# print(np.resize(results, (max_horizontal, max_vertial)).T)
print(results.count(1))
raise
available_groups = set(range(len(data)))
finite_groups = available_groups - infinite_groups

print(available_groups)
print(infinite_groups)
print(finite_groups)


print(sorted(list(map(lambda i:[i ,results.count(i)], finite_groups)), key=lambda x:x[1]))
# abc_res = [string.ascii_lowercase[i] for i in results]
