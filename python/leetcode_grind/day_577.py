# https://leetcode.com/problems/sort-colors/editorial
class Solution1:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        i = 0
        j = 0
        k = len(nums) - 1 
        while j <= k:
            if nums[j] == 0:
                t = nums[i]
                nums[i] = nums[j]
                nums[j] = t
                i += 1
                j += 1
            elif nums[j] == 1:
                j+=1
            else:
                t = nums[j]
                nums[j] = nums[k]
                nums[k] = t
                k -= 1

# https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/description/
class Solution2:
    def numberOfSubstrings(self, s: str) -> int:
        cnt = [0] * 26
        for ch in s:
            cnt[ord(ch) - ord('a')] += 1
        ans = 0
        for c in cnt:
            ans += c * (c + 1) // 2
        return ans
        
# https://leetcode.com/problems/minimum-increment-to-make-array-unique/description
class Solution3:
    def minIncrementForUnique(self, nums: List[int]) -> int:
        min_increments = 0
        nums.sort()
        for i in range(1, len(nums)):
            if nums[i] <= nums[i - 1]:
                increment = nums[i - 1] + 1 - nums[i]
                min_increments += increment
                nums[i] = nums[i - 1] + 1
        return min_increments