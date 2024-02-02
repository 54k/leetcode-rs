# https://leetcode.com/problems/sequential-digits/
class Solution1:
    def sequentialDigits(self, low: int, high: int) -> List[int]:
        sample = "123456789"
        nums = []

        for l in range(2, 10):
            for start in range(0, 10 - l):
                nums.append(int(sample[start:start+l]))

        return [x for x in nums if low <= x <= high]

# https://leetcode.com/problems/sequential-digits/
class Solution2:
    def sequentialDigits(self, low: int, high: int) -> List[int]:
        sample = "123456789"
        nums = []

        for l in range(2, 10):
            for start in range(0, 10 - l):
                nums.append(int(sample[start:start+l]))

        return [x for x in nums if low <= x <= high]