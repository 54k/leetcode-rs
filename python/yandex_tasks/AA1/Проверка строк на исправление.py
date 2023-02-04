# Нужно реализовать функцию OneEditApart , проверяющую, можно ли одну строку получить из другой не более,
# чем за одно исправление (удаление, добавление, изменение символа):

def OneEditApart(a, b):
    a_len = len(a)
    b_len = len(b)

    if abs(a_len - b_len) > 1:  # фильтруем заведомо фейл чтобы дальше не крутить
        return False
    elif a_len == b_len:  # если нужно изменение символа
        has_diff = False
        for i in range(a_len):
            if a[i] != b[i]:
                if has_diff:
                    return False
                has_diff = True
        return True
    elif a_len < b_len:  # чтобы считать а большей строкой
        a_len, b_len = b_len, a_len
        a, b = b, a

    # алгоритм из fuzzy search)
    i = 0
    for j in a:
        if j == b[i]:
            i += 1
            if i == b_len:  # Если нашли всё слово
                return True
    return False
