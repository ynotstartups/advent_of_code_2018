def diff_polarity(e0, e1):
    # make a, A and A, a to a, A
    e0, e1 = sorted([e0, e1], reverse=True)
    if e0.lower() != e1.lower():
        return False
    elif e0 == e1:
        return False
    elif e0 == e1.lower():
        return True
    else:
        __import__('pdb').set_trace()
        raise('imposible')

assert diff_polarity('a', 'a') is False
assert diff_polarity('a', 'b') is False
assert diff_polarity('A', 'A') is False
assert diff_polarity('a', 'A') is True
assert diff_polarity('A', 'a') is True
assert diff_polarity('a', 'B') is False

data = 'dabAcCaCBAcCcaDA'

data = open("input.txt", "r").read().strip()


print(len(data))

# todo make sure the stop statement is corrent
def react(data):
    index = 0
    while index < len(data) - 1:
        current_element = data[index]
        last_element = data[index + 1]

        if diff_polarity(current_element, last_element):
            data = data[:index] + data[index+2:]
            if index != 0:
                index -= 1
            continue
        else:
            pass
        index += 1

    return len(data)

import string

print(data)
print()
for i in string.ascii_lowercase:
    i_data = data.replace(i, '')
    i_data = i_data.replace(i.upper(), '')

    print(i, react(i_data))
