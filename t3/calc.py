import re

if __name__ == "__main__":
    res = {}
    with open('./data.txt', 'r+') as datas:
        for data in datas:
            _, col_start, row_start, col_width, row_width = [int(i) for i in re.split('[\D]', data) if i != '']
            col_start += 1
            row_start += 1
            for col in range(col_start, col_start+col_width):
                for row in range(row_start, row_start+row_width):
                    index = (col, row)
                    if index not in res:
                        res[index] = 0
                    else:
                        res[index] += 1

    clan_score = {}
    with open('./data.txt', 'r+') as datas:
        # which clans only has no overlap
        for data in datas:
            clan, col_start, row_start, col_width, row_width = [int(i) for i in re.split('[\D]', data) if i != '']
            col_start += 1
            row_start += 1

            clan_score[clan] = 0
            for col in range(col_start, col_start+col_width):
                for row in range(row_start, row_start+row_width):
                    index = (col, row)
                    clan_score[clan] += res[index]

            if clan_score[clan] == 0:
                print(clan_score, clan)



    print('sum', sum([1 for i in res.values() if i > 0]))
