# Дана строка из десятичных цифр (длинное число, младшие разряды расположены по младшему индексу).
# Написать код, который умножит это число на число 1 <= n <= 9.
# Ограничения по памяти: O(1) дополнительной памяти,
# т.е. надо использовать исходную строку (считаем, что возможное увеличение длины
# на 1 разряд не приведёт к реаллокации).

from pprint import pprint


def multiply_string(st, m):
    st_list = list(st)
    overflow = 0
    for i in range(len(st_list)):
        current = int(st_list[i]) * m + overflow
        st_list[i] = str(current % 10)
        overflow = current // 10

    if overflow > 0:
        st_list.append(overflow)

    return ''.join(st_list)


print(multiply_string('95', '5'))
