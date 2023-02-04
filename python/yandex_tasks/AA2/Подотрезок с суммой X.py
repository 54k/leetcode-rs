#  Дан массив целых чисел, нужно найти непустой подотрезок
# (непрерывную подпоследовательность) с заданной суммой X, либо сказать, что это невозможно.
#  Для найденного отрезка (если он существует) следует выдать на выход индексы его концов.

#  Случай если надо отдать первый отрезок
from collections import defaultdict
from pprint import pprint
from typing import List


class Solution1:
    def subarraySum(nums: List[int], k: int) -> int:
        s = 0  # бегущая сумма
        m = defaultdict(int)
        for i in range(len(nums)):
            s += nums[i]
            if s == k:
                return 0, i;

            if s - k in m:
                return m[s - k] + 1, i
            m[s] = i
        return None


pprint(Solution1.subarraySum([1, 1, 1, 1, 4, 4, 2], 7))

#  Случай если надо отдать все отрезки (отдам листом пар индексов)
from collections import defaultdict


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        s = 0  # бегущая сумма
        m = defaultdict(list)
        result = []
        for i in range(len(nums)):
            m[i] = 1  # чтобы ловить подмассивы из одного символа
            s += nums[i]
            if s - k in m:
                for j in m[s - k]:
                    result.append(j, i)
            m[s].append(i)
        return result


# Решение для задачи "подсчитать количество таких отрезков"

from collections import defaultdict


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        count = 0  # количество нужных подотрезков
        s = 0  # бегущая сумма
        m = defaultdict(int)
        m[0] = 1  # чтобы ловить подмассивы из одного символа

        for n in nums:
            s += n  # докидываем текущий элемент в бегущую сумму
            count += m[s - k]  # если сущестовали элементы с текущей суммой минус k, докидываем их количество в резалт
            m[s] = m[s] + 1  # прибавляем количество элементов с текущей бегущей суммой

        return count
