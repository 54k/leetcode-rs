#  Дан текст T и строка S. Требуется найти подстроку S' в T такую,
#  что она совпадает с S с точностью до перестановки букв.

from collections import defaultdict
from pprint import pprint


class Solution:
    def checkInclusion(s1: str, s2: str) -> bool:
        s1_len = len(s1)
        window = defaultdict(int)

        for s in s1:
            window[s] += 1  # Положительне значения -- недостающие значения в окне

        for s in s2[0:s1_len]:
            window[s] -= 1  # Отрицательные значения -- лишние символы в окне
            if window[s] == 0:
                window.pop(s)  # после удалений проверяем ключ на нулевость, если пустой, удаляем

        for i in range(0, len(s2) - s1_len):
            if not window:  # если в окне не осталось ключей-значений, то окно совпало с искомой подстрокой
                return True
            window[s2[i]] += 1  # этот символ покидает окно
            window[s2[i + s1_len]] -= 1  # этот символ попадает в окно
            if window[s2[i]] == 0:
                window.pop(s2[i])
            if window[s2[i + s1_len]] == 0:
                window.pop(s2[i + s1_len])
        if not window:  # проверим на пустоту последнее возможное окно
            return True
        return False  # если до этого момента не ретернули тру, то в строке нет нужной подстроки


pprint(Solution.checkInclusion("aabcdefgjk", "aabczeraaaabcdaekfgjzpi"))
