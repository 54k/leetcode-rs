# Дан вектор, надо удалить из него нули, сохранив порядок остальных элементов.
# Интересует как с использованием стандартных средств, так и без них.
from typing import List


# Для языков типа Python или Java, где управление памятью не так очевидно,
# можно использовать формулировку "перенести нули в конец массива" вместо удаления.

class Solution:
    def moveZeroes(nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        zeroes_count = 0

        for i in range(len(nums)):
            n = nums[i]
            if n == 0:
                zeroes_count += 1
            else:
                nums[i], nums[i - zeroes_count] = nums[i - zeroes_count], nums[i]
        return nums


#  пижонство)
class Solution1(object):
    def moveZeroes(nums: List[int]) -> None:
        """
        :type nums: List[int]
        :rtype: void Do not return anything, modify nums in-place instead.
        """
        return nums.sort(cmp=lambda a, b: 0 if b else -1)


print(Solution.moveZeroes(list([0, 0, 0, 0, 1, 1, 2, 2, 3, 1, 4, 0, 4, 0])))
