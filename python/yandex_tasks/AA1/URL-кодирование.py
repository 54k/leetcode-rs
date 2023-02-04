# Inplace заменяет в массиве символы пробела на последовательность символов '%', '2', '0'.


from pprint import pprint


def urlify(st):
    st_l = list(st)
    i = len(st_l) - 1

    spaces_count = 0
    for s in st_l:
        if s == ' ':
            spaces_count += 1

    st_l += [''] * spaces_count * 2  # added extra space

    j = len(st_l) - 1
    while i >= 0:
        if st_l[i] == ' ':
            st_l[j] = '0'
            j -= 1
            st_l[j] = '2'
            j -= 1
            st_l[j] = '%'
        else:
            st_l[j] = st_l[i]

        j -= 1
        i -= 1
    return ''.join(st_l)


s0 = " my  ur l "

pprint(urlify(s0))
