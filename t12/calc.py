def current_plant_key(index, state):
    key = [i for i in "....."]

    try:
        key[0] = state[index-2]
    except IndexError:
        pass

    try:
        key[1] = state[index-1]
    except IndexError:
        pass

    try:
        key[2] = state[index]
    except IndexError:
        pass

    try:
        key[3] = state[index+1]
    except IndexError:
        pass

    try:
        key[4] = state[index+2]
    except IndexError:
        pass
    
    key = "".join(key)
    return key

assert current_plant_key(0, "#..") == "..#.."
assert current_plant_key(0, ".#..") == "...#."
assert current_plant_key(2, "..#.") == "..#.."
assert current_plant_key(2, "..#") == "..#.."
assert current_plant_key(2, "..#..") == "..#.."

data = """
initial state: #..#.#..##......###...###\n
\n
...## => #\n
..#.. => #\n
.#... => #\n
.#.#. => #\n
.#.## => #\n
.##.. => #\n
.#### => #\n
#.#.# => #\n
#.### => #\n
##.#. => #\n
##.## => #\n
###.. => #\n
###.# => #\n
####. => #\n
"""

data = open("./input.txt", "r+").read().strip()

INITIAL_STATE = "initial state: "
ARROW = " => "

data = data.split('\n')

state = ''
evolve_map = {}

for line in data:
    if line == '':
        pass
    elif line.startswith(INITIAL_STATE):
        state = line.replace(INITIAL_STATE, '')
    elif ARROW in line:
        k, v = line.split(ARROW)
        evolve_map[k] = v
    else:
        assert True, 'should never be here'


assert state
assert evolve_map
print(state)
print(evolve_map)

zero_index = 0
_last_sum = 0
for i in range(500):
    state = ".." + state + ".."
    state_list = [i for i in state]
    for current_plant_index in range(len(state)):
        key = current_plant_key(current_plant_index, state)
        # state_list[current_plant_index] = evolve_map[key]
        if key in evolve_map:
            state_list[current_plant_index] = evolve_map[key]
        else:
            state_list[current_plant_index] = '.'
    
    state = "".join(state_list)
    zero_index += 2

    _sum = 0
    state_list = [s for s in state]
    for state_index in range(len(state_list)):
        if state_list[state_index] == "#":
            _sum += state_index - zero_index

    try:
        print(_sum - _last_sum, _last_sum, _sum, i)
        _last_sum = _sum
    except:
        pass

