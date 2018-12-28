from llist import dllist, dllistnode
from itertools import cycle

def _next_node(current_node):
    if current_node.next is not None:
        return current_node.next
    else:
        return marbles_circle.first

def _previous_node(current_node):
    if current_node.prev is not None:
        return current_node.prev
    else:
        return marbles_circle.last

PLAYER_NUMBER = 464
MARBLE_NUMBER = 7173000

c_player = cycle(range(1, PLAYER_NUMBER+1))
player_scores = {i : 0 for i in range(1, PLAYER_NUMBER+1)}

marbles_circle = dllist([0])
first_node = marbles_circle.nodeat(0)
current_marble_node = marbles_circle.nodeat(0)

for current_marble, player in zip(range(1, MARBLE_NUMBER + 1), cycle(c_player)):
    print(current_marble)

    if current_marble % 23 != 0:
        current_marble_node = _next_node(_next_node(current_marble_node))

        if current_marble_node is first_node:
            current_marble_node = marbles_circle.append(current_marble)
        else:
            current_marble_node = marbles_circle.insert(current_marble, current_marble_node)
    else:
        for i in range(7):
            current_marble_node = _previous_node(current_marble_node)
        

        poped_marble = current_marble_node
        current_marble_node = _next_node(current_marble_node)
        poped = marbles_circle.remove(poped_marble)
        player_scores[player] += current_marble + poped

print(sorted(player_scores.items(), key=lambda x:x[1], reverse=True)[0][1])

