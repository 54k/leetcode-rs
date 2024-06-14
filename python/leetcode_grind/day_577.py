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