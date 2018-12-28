from itertools import cycle
def next_index(current_index, len_iterable):
    if len_iterable > current_index + 1:
        return current_index + 1
    else:
        return 0

def previous_index(current_index, len_iterable):
    if current_index - 1 >= 0:
        return current_index - 1
    else:
        return len_iterable - 1

assert next_index(0, 0) == 0
assert next_index(0, 1) == 0
assert next_index(1, 2) == 0
assert next_index(1, 3) == 2
assert next_index(3, 4) == 0

# does not work on small number
# assert previous_index(0, 0) == 0
# assert previous_index(0, 1) == 1
assert previous_index(1, 2) == 0
assert previous_index(0, 3) == 2
assert previous_index(4, 4) == 3


PLAYER_NUMBER = 13
# worth is different
MARBLE_NUMBER = 200

current_marble_index = 0
marbles_circle = [0]
c_player = cycle(
    list(range(7, PLAYER_NUMBER+1)) + list(range(1, 7))
)

player_scores = {i : 0 for i in range(1, PLAYER_NUMBER+1)}
player_scores[1] = 32


player_marble_poped = []

FIRST_TIME = True
marbles_circle = [42, 4, 43, 18, 44, 19, 45, 2, 24, 20, 25, 10, 26, 21, 27, 5, 28, 22, 29, 11, 30, 1, 31, 12, 32, 6, 33, 13, 34, 3, 35, 14, 36, 7, 37, 15, 38, 0, 39, 16, 40, 8, 41]
for current_marble, player in zip(range(47, MARBLE_NUMBER + 1), c_player):

    print('cmi', marbles_circle)


    if current_marble % 23 != 0:
        current_marble_index = (current_marble % 23) * 2
        marbles_circle.insert(current_marble_index, current_marble)

        assert current_marble_index < len(marbles_circle), f'{current_marble_index}, {len(marbles_circle)}'
    else:
        if FIRST_TIME:
            current_marble_index = 22 * 2 - 7
            FIRST_TIME = False
        else:
            current_marble_index = 22 * 2 - 7
        assert current_marble_index < len(marbles_circle)

        poped = marbles_circle.pop(current_marble_index)
        print(f'poping {poped}')
        player_marble_poped.append([current_marble_index, poped])
        player_scores[player] += current_marble + poped

        marbles_circle = marbles_circle[current_marble_index-1:] + marbles_circle[:current_marble_index-1]

print(sorted(player_scores.items(), key=lambda x:x[1], reverse=True)[0][1])




    

