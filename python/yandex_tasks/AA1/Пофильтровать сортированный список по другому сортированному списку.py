from pprint import pprint


def sorted_filter(it1, it2):
    cur1 = next(it1, None)
    cur2 = next(it2, None)
    while cur1 is not None:
        if cur2 is None or cur1 < cur2:
            yield cur1
            cur1 = next(it1, None)
        elif cur1 == cur2:
            cur1 = next(it1, None)
        elif cur1 > cur2:
            cur2 = next(it2, None)


def s_f2(it1, it2):
    c1 = it1[0] if it1 else None
    c2 = it2[0] if it2 else None

    result = []

    while c1 is not None:
        if c2 is None or c1 < c2:
            result.append(it1.pop(0))
            c1 = it1[0] if it1 else None
        elif c1 == c2:
            it1.pop(0)
            c1 = it1[0] if it1 else None
        elif c1 > c2:
            it2.pop(0)
            c2 = it2[0] if it2 else None

    return result


pprint(s_f2([1, 3, 5, 7, 8, 9], [1, 2, 6, 8, 9]))
