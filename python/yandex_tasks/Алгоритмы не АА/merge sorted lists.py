def merge_sorted_lists(l1, l2):
    """Merge sort two sorted lists

    Arguments:
    - `l1`: First sorted list
    - `l2`: Second sorted list
    """
    sorted_list = []

    # Copy both the args to make sure the original lists are not
    # modified
    l1 = l1[:]
    l2 = l2[:]

    while (l1 and l2):
        if (l1[0] <= l2[0]):  # Compare both heads
            item = l1.pop(0)  # Pop from the head
            sorted_list.append(item)
        else:
            item = l2.pop(0)
            sorted_list.append(item)

    # Add the remaining of the lists
    sorted_list.extend(l1 if l1 else l2)

    return sorted_list
