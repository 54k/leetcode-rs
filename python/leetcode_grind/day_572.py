# https://leetcode.com/problems/count-number-of-bad-pairs/description/
class Solution1:
    def countBadPairs(self, nums: List[int]) -> int:
        cnt = {}
        good = 0
        for i in range(len(nums)):
            if not nums[i] - i in cnt:
                cnt[nums[i] - i] = 0
            good += cnt[nums[i] - i]
            cnt[nums[i] - i] += 1
        n = len(nums)
        return n * (n-1) // 2 - good