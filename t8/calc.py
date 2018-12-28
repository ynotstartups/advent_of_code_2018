# data = [2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2]

data = open('./input.txt', 'r+').read().strip()
data = [int(i) for i in data.split(' ')]

sum_metadata = 0

HEADER_LENGTH = 2

# strategy whenever I find a 0 node remove them from data
# and reduce the number of child in the correct position
# and add metadata whenever encourter a 0 node

# start_index is global
# start_index = 0
# while True:
#     number_child = data[start_index]
#     number_metadata = data[start_index + 1]

#     if number_child == 0:
#         sum_metadata += sum(data[start_index + HEADER_LENGTH:start_index+HEADER_LENGTH+number_metadata])
#         data = data[:start_index] + data[start_index+HEADER_LENGTH+number_metadata:]

#         if start_index != 0:
#             data[start_index - 2] -= 1
#             start_index -= 2
#         else:
#             # finished
#             break
#     else:
#         start_index += 2

print(sum_metadata)

group_children_metadata = {}
current_group = 0
start_index = 0
global_group_counter = 0
group_children_metadata[0] = [[], None]
while True:
    number_child = data[start_index]
    number_metadata = data[start_index + 1]

    if number_child == 0:
        metadata = data[start_index + HEADER_LENGTH:start_index+HEADER_LENGTH+number_metadata]
        data = data[:start_index] + data[start_index+HEADER_LENGTH+number_metadata:]

        # assert len(group_children_metadata[current_group]) == 2
        group_children_metadata[current_group] += metadata

        if start_index != 0:
            data[start_index - 2] -= 1
            start_index -= 2
            
            parent_group = group_children_metadata[current_group][1]
            group_children_metadata[parent_group][0].append(current_group)
            current_group = parent_group
        else:
            # finished
            break
    else:
        start_index += 2
        parent_group = current_group

        # new group counter
        global_group_counter += 1
        current_group = global_group_counter

        group_children_metadata[current_group] = [[], parent_group]

print(group_children_metadata)

# part 2 calc
group_meta_value = {}
while 0 not in group_meta_value:
    print(len(group_meta_value), group_meta_value)
    for group, children_parent_metadata in group_children_metadata.items():
        if group in group_meta_value:
            continue
        if children_parent_metadata[0] == []: # group with no children
            metavalue = sum(children_parent_metadata[2:])
            if group not in group_meta_value:
                group_meta_value[group] = metavalue
        else:
            finished = True
            metadatas = children_parent_metadata[2:]
            metavalue = 0
            for metadata in metadatas:
                if group == 4:
                    print(f'4 metadata {metadata}')
                if metadata > len(children_parent_metadata[0]):
                    continue

                print(metadatas, metadata-1)
                need_metavalue_group = children_parent_metadata[0][metadata-1] 

                if need_metavalue_group not in group_meta_value:
                    finished = False
                    break
                else:
                    metavalue += group_meta_value[need_metavalue_group]

            if finished:
                group_meta_value[group] = metavalue



print(group_meta_value[0])
