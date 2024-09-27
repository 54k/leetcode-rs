# https://leetcode.com/problems/count-of-range-sum/description/
class Solution:
    def countRangeSum(self, nums: List[int], lower: int, upper: int) -> int:
        prefix = nums.copy()
        for i in range(1, len(nums)):
            prefix[i] += prefix[i-1]
        self.count = 0
        for n in prefix:
            if lower <= n <= upper:
                self.count += 1
        
        def merge(left_arr, right_arr):
            start, end = 0, 0
            for i in range(len(left_arr)):
                while start < len(right_arr) and right_arr[start] - left_arr[i] < lower:
                    start += 1
                while end < len(right_arr) and right_arr[end] - left_arr[i] <= upper:
                    end += 1
                self.count += end - start
            
            l, r = 0, 0
            sorted_arr = []
            while l < len(left_arr) and r < len(right_arr):
                if left_arr[l] < right_arr[r]:
                    sorted_arr.append(left_arr[l])
                    l += 1
                else:
                    sorted_arr.append(right_arr[r])
                    r += 1
            return sorted_arr + left_arr[l:] + right_arr[r:]
        
        def divide(arr):
            if len(arr) <= 1: return arr
            mid = len(arr) // 2
            left_arr = divide(arr[:mid])
            right_arr = divide(arr[mid:])
            return merge(left_arr, right_arr)
        
        divide(prefix)
        return self.count