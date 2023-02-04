def find_intersection(l1, l2):
    out = []
    i1 = 0
    i2 = 0
    while (i1 < len(l1)) and (i2 < len(l2)):
        if l1[i1] > l2[i2]:
            i2 += 1
        elif l1[i1] < l2[i2]:
            i1 += 1
        else:  # l1[i1] == l2[i2]
            out.append(l1[i1])
            i1 += 1
            i2 += 1
    return out
