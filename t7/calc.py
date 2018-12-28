data = [
    'Step C must be finished before step A can begin.',
    'Step C must be finished before step F can begin.',
    'Step A must be finished before step B can begin.',
    'Step A must be finished before step D can begin.',
    'Step B must be finished before step E can begin.',
    'Step D must be finished before step E can begin.',
    'Step F must be finished before step E can begin.',
]

data = []

with open('./input.txt', 'r+') as f:
    for line in f:
        data.append(line)

import re
import string

two_letters = re.compile(r".*([A-Z]).*([A-Z]).*")

target_requirements = {}
for line in data:
    requirement, target = re.match(two_letters, line).group(1,2)

    if target not in target_requirements:
        target_requirements[target] = [requirement]
    else:
        assert requirement not in target_requirements[target]
        target_requirements[target].append(requirement)

    if requirement not in target_requirements:
        target_requirements[requirement] = []

print(target_requirements)

def pop_letter(letter, target_requirements):
    target_requirements.pop(letter)

    for target in target_requirements:
        requirement = target_requirements[target]
        if letter in requirement:
            requirement.remove(letter)


def ready_to_do(target_requirements):
    return sorted([target for target, requirement in target_requirements.items() if requirement == []])

# order = ''

# while target_requirements:
#     next_letter = ready_to_do(target_requirements)[0]
#     target_requirements.pop(next_letter)
#     order += next_letter
#     pop_letter(next_letter, target_requirements)

# print(order)

def time_for_target(target):
    return string.ascii_uppercase.index(target) + 1

assert time_for_target('A') == 1
assert time_for_target('C') == 3
assert time_for_target('Z') == 26

def remove_one_second_build_time(worker):
    if worker is None:
        return None
    else:
        target, second_remaining = worker
        if second_remaining == 1:
            return target
        else:
            return [target, second_remaining - 1]
            
UPPERCASE_LIST = [i for i in string.ascii_uppercase]
WORKER_NUMBER = 5
EXTRA_SECOND = 60

workers_task = [None for i in range(WORKER_NUMBER)]
# task is like [targer, second_remaining]
second = -1
while target_requirements:

    for index, worker in enumerate(workers_task):
        res = remove_one_second_build_time(worker)
        # worker was and is still idle
        if res is None:
            continue

        # one target is finished
        if res in UPPERCASE_LIST:
            print(f'{second} popping {res} {target_requirements} {workers_task}')
            pop_letter(res, target_requirements)
            workers_task[index] = None
            continue

        if isinstance(res, list):
            workers_task[index] = res
            continue

        assert False, f'it should never be here {index} {worker}'

    can_start_targets = ready_to_do(target_requirements)
    for started_target in [i[0] for i in workers_task if i is not None]:
        if started_target in can_start_targets:
            can_start_targets.remove(started_target)

    print(can_start_targets)
    for target in can_start_targets:
        for index, worker in enumerate(workers_task):
            # idle
            if worker is None:
                workers_task[index] = [target, EXTRA_SECOND + time_for_target(target)]
                break

    # print(workers_task)

    second += 1

print(second)


