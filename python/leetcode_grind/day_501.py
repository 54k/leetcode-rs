# https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/description/
from collections import Counter 

class Solution:
    def maxSubarrayLength(self, nums: List[int], k: int) -> int:
        cnt = Counter()
        ans, l = 0, 0
        for r in range(len(nums)):
            cnt[nums[r]] += 1
            while l < r and cnt[nums[r]] > k:
                cnt[nums[l]] -= 1
                l+=1
            ans = max(ans, r - l + 1)
        return ans
