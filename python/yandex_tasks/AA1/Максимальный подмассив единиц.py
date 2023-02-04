# Дан массив из нулей и единиц. Нужно определить, какой максимальный по длине
# подинтервал единиц можно получить, удалив ровно один элемент массива.
from typing import List


class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        result = 0
        is_zero_in_array = False
        currentLength = 0
        before_last_zero = 0

        for n in nums:
            if n == 0:
                is_zero_in_array = True
                before_last_zero = currentLength  # Если два или больше раза подряд ловим ноль, то бефо ласт тоже сбрасывается
                currentLength = 0
            else:
                currentLength += 1
                if before_last_zero + currentLength > result:
                    result = before_last_zero + currentLength

        if result and not is_zero_in_array:  # проверка на случай если весь массив -- единицы
            result -= 1
        return result
