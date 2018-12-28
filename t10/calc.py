import numpy as np
import matplotlib.pyplot as plt

data = [
        'position=< 9,  1> velocity=< 0,  2>',
        'position=< 7,  0> velocity=<-1,  0>',
        'position=< 3, -2> velocity=<-1,  1>',
        'position=< 6, 10> velocity=<-2, -1>',
        'position=< 2, -4> velocity=< 2,  2>',
        'position=<-6, 10> velocity=< 2, -2>',
        'position=< 1,  8> velocity=< 1, -1>',
        'position=< 1,  7> velocity=< 1,  0>',
        'position=<-3, 11> velocity=< 1, -2>',
        'position=< 7,  6> velocity=<-1, -1>',
        'position=<-2,  3> velocity=< 1,  0>',
        'position=<-4,  3> velocity=< 2,  0>',
        'position=<10, -3> velocity=<-1,  1>',
        'position=< 5, 11> velocity=< 1, -2>',
        'position=< 4,  7> velocity=< 0, -1>',
        'position=< 8, -2> velocity=< 0,  1>',
        'position=<15,  0> velocity=<-2,  0>',
        'position=< 1,  6> velocity=< 1,  0>',
        'position=< 8,  9> velocity=< 0, -1>',
        'position=< 3,  3> velocity=<-1,  1>',
        'position=< 0,  5> velocity=< 0, -1>',
        'position=<-2,  2> velocity=< 2,  0>',
        'position=< 5, -2> velocity=< 1,  2>',
        'position=< 1,  4> velocity=< 2,  1>',
        'position=<-2,  7> velocity=< 2, -2>',
        'position=< 3,  6> velocity=<-1, -1>',
        'position=< 5,  0> velocity=< 1,  0>',
        'position=<-6,  0> velocity=< 2,  0>',
        'position=< 5,  9> velocity=< 1, -2>',
        'position=<14,  7> velocity=<-2,  0>',
        'position=<-3,  6> velocity=< 2, -1>',
        ]

data = []
with open('./input.txt', 'r+') as f:
    for l in f:
        data.append(l)

import re
number_pattern = '\s*(-?\d*)'
po_ve_pattern = re.compile(f'^position=<{number_pattern},{number_pattern}> velocity=<{number_pattern},{number_pattern}>')
def _max_data(position_velocities):
    _max_x = max([i[0] for i in position_velocities])
    _max_y = max([i[1] for i in position_velocities])

    return max(_max_x, _max_y)

position_velocities = []
for line in data:
    position_velocities.append([int(i) for i in re.match(po_ve_pattern, line).group(1, 2, 3, 4)])
i = 0
while True:
    
    i += 1
    should_plot = False
    print(i, _max_data(position_velocities))
    if i == 10476:
        should_plot = True
        fig = plt.figure()
        ax = fig.add_subplot(1, 1, 1)
        _map = [[' '] * 150 for j in range(150)]

    for index, position_velocity in enumerate(position_velocities):

        position_velocity[0] += position_velocity[2]
        position_velocity[1] += position_velocity[3]

        position_velocities[index] = position_velocity
        
        if should_plot:
            _map[position_velocity[1]-75][position_velocity[0]-75] = "*"

    if should_plot:
        for line in _map:
            print("".join(line))
        break





    
