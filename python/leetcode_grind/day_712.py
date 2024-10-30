# https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/description/
class Solution:
    def smallestRange(self, nums: List[List[int]]) -> List[int]:
        merged = []
        for i in range(len(nums)):
            for num in nums[i]:
                merged.append((num, i))
        
        merged.sort()

        freq = defaultdict(int)
        left, count = 0, 0
        range_start, range_end = 0, float("inf")

        for right in range(len(merged)):
            freq[merged[right][1]] += 1
            if freq[merged[right][1]] == 1:
                count += 1
            
            while count == len(nums):
                cur_range = merged[right][0] - merged[left][0]
                if cur_range < range_end - range_start:
                    range_start = merged[left][0]
                    range_end = merged[right][0]
                
                freq[merged[left][1]] -= 1
                if freq[merged[left][1]] == 0:
                    count -= 1
                left += 1

        return [range_start, range_end]