#  Дан список интов, повторяющихся элементов в списке нет.
#  Нужно преобразовать это множество в строку, сворачивая соседние по числовому ряду числа в диапазоны.

class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        nums.sort()

        result = []
        last_num = None
        last_start = None

        for n in nums:
            if last_start is None:
                last_start = n
                last_num = n
                continue

            if last_num + 1 == n:
                last_num = n
                continue

            if last_num == last_start:
                result.append(str(last_num))
            else:
                result.append(f"{last_start}-{last_num}")
            last_start = n
            last_num = n

        if last_num is not None:
            if last_num == last_start:
                result.append(str(last_num))
            else:
                result.append(f"{last_start}-{last_num}")

        return ','.join(result)


#  С литкода

def summaryRanges(self, nums):
    nums.sort()
    ranges, r = [], []
    for n in nums:
        if n - 1 not in r:  # r contains at most two elements, so the in-check takes constant time.
            r = []
            ranges += r,
        r[1:] = n,
    return ['-'.join(map(str, r)) for r in ranges]
