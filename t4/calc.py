import re
from collections import Counter
from datetime import datetime
from operator import itemgetter

first_item = itemgetter(0)

re_date = re.compile('\[(.*)\]')
re_falls_asleep = re.compile('falls asleep')
re_wakes_up = re.compile('wakes up')
re_guard_number = re.compile('#(\d+)')

if __name__ == "__main__":
    datas = []
    with open('./data.txt', 'r+') as f:
        for row in f:
            data = []
            datas.append(data)
            date = re.search(re_date, row).group(1)
            date = datetime.strptime(date, '%Y-%m-%d %H:%M')
            guard_number = re.search(re_guard_number, row)

            data.append(date)

            if re.search(re_falls_asleep, row):
                data.append('f')
            elif re.search(re_wakes_up, row):
                data.append('w')
            elif guard_number:
                guard_number = int(re.search(re_guard_number, row).group(1))
                data.append(guard_number)
            else:
                raise

            print(data)
            assert len(data) == 2

    print(datas)
    datas = iter(sorted(datas, key=first_item))

    guard_sleep_counter = {}
    current_guard_number = None
    for row in datas:
        date, identifier = row
        if isinstance(identifier, int):
            current_guard_number = identifier
            if current_guard_number not in guard_sleep_counter:
                guard_sleep_counter[current_guard_number] = []
        elif identifier == 'f':
            fall_asleep_minute = date.minute
            wake_row = next(datas)
            wake_minute = wake_row[0].minute
            guard_sleep_counter[current_guard_number] += range(fall_asleep_minute, wake_minute)
        elif identifier == 'w':
            raise
        else:
            raise

    # guard_sleep_counter_array = sorted(guard_sleep_counter.items(), key = lambda x:len(x[1]), reverse=True)

    # TODO
    # map to (guard_number, sleep_minutes, count)
    # sorted by count

    def gn_sl_count(each_guard_sleep_minutes):
        guard_number, sleep_minutes = each_guard_sleep_minutes
        most_common = Counter(sleep_minutes).most_common()
        if most_common:
            sleep_minute, max_count = most_common[0]
        else:
            sleep_minute = 999
            max_count = 0
        return guard_number, sleep_minute, max_count

    gn_sl_count_list = map(gn_sl_count, guard_sleep_counter.items())
    print(sorted(gn_sl_count_list, key=lambda x:x[2], reverse=True))
