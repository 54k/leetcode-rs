# https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description/
class Solution:
    def maximumValueSum(self, nums: List[int], k: int, edges: List[List[int]]) -> int:
        n = len(nums)
        netChange =  [(nums[i] ^ k) - nums[i] for i in range(n)]
        nodeSum = sum(nums)
        netChange.sort(reverse=True)
        for i in range(0, n, 2):
            if i + 1 == n:
                break
            pairSum = netChange[i] + netChange[i + 1] 
            if pairSum > 0:
                nodeSum += pairSum
        return nodeSum