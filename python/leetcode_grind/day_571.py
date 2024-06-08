# https://leetcode.com/problems/continuous-subarray-sum/description
class Solution1:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        prefix_mod = 0
        mod_seen = {0: -1}
        for i in range(len(nums)):
            prefix_mod = (prefix_mod + nums[i]) % k
            if prefix_mod in mod_seen:
                if i - mod_seen[prefix_mod] > 1:
                    return True
            else:
                mod_seen[prefix_mod] = i
        return False

# https://leetcode.com/problems/subarray-sum-equals-k/description/
class Solution2:
    def subarraySum(self, nums: List[int], k: int) -> int:
        m = {0: 1}
        s = 0
        ans = 0
        for n in nums:
            s += n
            if s - k in m:
                ans += m[s-k]
            if not s in m:
                m[s] = 0
            m[s] += 1
        return ans

# https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/description/
class Solution3:
    def minOperations(self, nums: List[int]) -> int:
        n = len(nums)
        ans = n
        new_nums = sorted(set(nums))
        j = 0
        for i in range(len(new_nums)):
            while j < len(new_nums) and new_nums[j] < new_nums[i] + n:
                j += 1
            count = j - i
            ans = min(ans, n - count)
        return ans