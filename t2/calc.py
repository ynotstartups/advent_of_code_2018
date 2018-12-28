from collections import Counter


def n_of_a_letter(chars):
    contains_two = False
    contains_three = False
    c = Counter(chars)
    for _, number in c.most_common():
        if number == 2:
            contains_two = True
        elif number == 3:
            contains_three = True
    return contains_two, contains_three


contains_2_of_a_letter = 0
contains_3_of_a_letter = 0

all_chars = []
with open('./data.txt', 'r+') as f:
    for chars in f:
        all_chars.append(chars)
        c_2, c_3 = n_of_a_letter(chars)
        contains_2_of_a_letter += c_2
        contains_3_of_a_letter += c_3
print(contains_2_of_a_letter*contains_3_of_a_letter)

# all_chars = [
# 'abcde',
# 'fghij',
# 'klmno',
# 'pqrst',
# 'fguij',
# 'axcye',
# 'wvxyz'
# ]
min_length = -9999
for i in all_chars:
    for j in all_chars:
        if i == j:
            continue
        res = ''
        for (c_i, c_j) in zip(i, j):
            if c_i == c_j:
                res += c_i
        length = len(res)
        if min_length < length:
            min_length = length
            print(res, length, i, j)
