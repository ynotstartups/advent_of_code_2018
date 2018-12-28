SERIAL_NUMBER = 42

def cell_power(x, y, serial_number):
    rack_ID = x + 10
    res = rack_ID
    res = res * y
    res = res + serial_number
    res = res * rack_ID

    # get hundred digit
    res = int(str(res)[-3])

    res = res - 5
    return res


assert cell_power(  3,   5,  8) == 4
assert cell_power(122,  79, 57) == -5
assert cell_power(217, 196, 39) == 0
assert cell_power(101, 153, 71) == 4

def sum_grid(x, y, size, matrix):
    _sum = 0
    for i in range(x-size, x+size+1):
        for j in range(y-size, y+size+1):
            _sum += matrix[i][j]
    return _sum

from functools import partial

cell_power_mine = partial(cell_power, serial_number=SERIAL_NUMBER)

_max = 300
matrix = []
for i in range(1, _max + 1):
    row = []
    matrix.append(row)
    for j in range(1, _max + 1):
        row.append(cell_power_mine(i, j))

    assert len(row) == _max

sum_grid_res = {}
for size in (1,3):
    for i in range(size + 2, _max-size):
        for j in range(size + 2, _max-size):
            sum_grid_res[(i,j, size)] = sum_grid(i, j, size, matrix)

print(sorted(sum_grid_res.items(), key=lambda x:x[1])[-10:])
assert len(matrix) == _max
