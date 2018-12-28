all_data = []
res = [0]
total = 0
with open('./input.txt', 'r+') as f:
    for i in f:
        all_data.append(int(i))

while True:
    for i in all_data:
        total += i
        res.append(total)
        print(total)
        if res.count(total) == 2:
            print('-----')
            print(res)
            print(total)
            raise
