from typing import List
from pprint import pprint;


# Дан отсортированный массив целых чисел a , целое число K и индекс элемента index.
# Необходимо вернуть в любом порядке K чисел из массива,
# которые являются ближайшими по значению к элементу a[index] .


class Solution:
    def findClosestElements(arr: List[int], k: int, x: int) -> List[int]:

        left = 0
        right = len(arr) - k  # max start point
        while left < right:
            mid = left + (right - left) // 2

            # mid + k is closer to x, discard mid by assigning left = mid + 1
            if arr[x] - arr[mid] > arr[mid + k] - arr[x]:
                left = mid + 1

            # mid is equal or closer to x than mid + k, remains mid as candidate
            else:
                right = mid

        # left == right, which makes both left and left + k have same diff with x
        return arr[left: left + k]


pprint(Solution.findClosestElements([1, 3, 5, 10, 15, 22, 23, 24, 25], 5, 8));
