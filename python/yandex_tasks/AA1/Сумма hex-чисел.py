# Даны две строки, каждая из которых — hex-число в прямой записи (старший знак в нулевой позиции).
# Написать код суммирования этих чисел.


from pprint import pprint


def hxe_sum(h1, h2):
    h1_len = len(h1)
    h2_len = len(h2)

    h_t_d = {
        '0': 0, '1': 1, '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7, '8': 8, '9': 9, 'A': 10,
        'B': 11, 'C': 12, 'D': 13, 'E': 14, 'F': 15
    }

    d_t_h = {value: key for key, value in h_t_d.items()}  # reverse mapping

    result = []
    overflow = 0

    for i in range(max(h1_len, h2_len)):
        if i < h1_len:
            h1_curr = h1[-1 - i]
        else:
            h1_curr = '0'  # zero if out of digits

        if i < h2_len:
            h2_curr = h2[-1 - i]
        else:
            h2_curr = '0'

        local = h_t_d[h1_curr] + h_t_d[h2_curr] + overflow

        result.append(d_t_h[local % 16])
        overflow = local // 16

    if overflow != '0':  # if extra overflowed
        result.append(d_t_h[overflow])

    return ''.join(result[::-1])  # reversing array O(n)


pprint(hxe_sum("FFFF", "1"))
