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


PLAYER_NUMBER = 464
# worth is different
MARBLE_NUMBER = 717300

current_marble_index = 0
marbles_circle = [0]
c_player = cycle(range(1, PLAYER_NUMBER+1))
player_scores = {i : 0 for i in range(1, PLAYER_NUMBER+1)}

length = 1

for current_marble, player in zip(range(1, MARBLE_NUMBER + 1), cycle(c_player)):

    if current_marble % 23 != 0:
        _next_index = next_index(current_marble_index, length)

        current_marble_index = _next_index + 1
        marbles_circle.insert(current_marble_index, current_marble)
        length += 1
    else:
        if current_marble_index - 7 <= 0:
            current_marble_index = previous_index(current_marble_index, length)
            current_marble_index = previous_index(current_marble_index, length)
            current_marble_index = previous_index(current_marble_index, length)
            current_marble_index = previous_index(current_marble_index, length)
            current_marble_index = previous_index(current_marble_index, length)
            current_marble_index = previous_index(current_marble_index, length)
            current_marble_index = previous_index(current_marble_index, length)
        else:
            current_marble_index = current_marble_index - 7

        
        poped = marbles_circle.pop(current_marble_index)
        player_scores[player] += current_marble + poped
        length -= 1

        # print(current_marble_index, len(marbles_circle))
        # if len(marbles_circle) > 200:
        #     marbles_circle = marbles_circle[10:]
        #     current_marble_index -= 10
        #     print(f'removing {current_marble_index} len {len(marbles_circle)}')


print(sorted(player_scores.items(), key=lambda x:x[1], reverse=True)[0][1])




    

