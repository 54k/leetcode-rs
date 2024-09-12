# https://leetcode.com/problems/maximum-sum-of-subsequence-with-non-adjacent-elements/description/
# https://leetcode.com/problems/maximum-sum-of-subsequence-with-non-adjacent-elements/solutions/5209280/Python-Square-Root-Decomposition.-No-Segment-Tree.-DP-of-DP.-O(MN).-Pass.-With-explanation/
import math

MOD = 1000000007
SECOND_LAST, LAST = 0, 1
USE_FIRST, SKIP_FIRST = 0, 1

class Solution:
    def maximumSumSubsequence(self, nums: List[int], queries: List[List[int]]) -> int:
        n = len(nums)        
        sqrtn = math.ceil(math.sqrt(n))

        blocks = [[] for _ in range(sqrtn)]
        for i in range(n):
            blocks[i // sqrtn].append(nums[i])

        def dp_in_block(ar, start_i): # DP in block. Returns the last two
            if len(ar) == 0 or (len(ar) == 1 and start_i == 1):
                return 0, 0
            elif len(ar) == 1:
                return 0, max(0, ar[0])
            elif len(ar) == 2 and start_i == 1:
                return 0, max(0, ar[1])
            
            dp0 = max(0, ar[start_i])
            dp1 = max(0, ar[start_i], ar[start_i+1])

            for i in range(start_i+2, len(ar)):
                dp0, dp1 = dp1, max(0, dp1, dp0+ar[i])

            return dp0, dp1
        
        # Cache for block sum Cache[block_i][use first or not] = last two dp elements
        cache = []
        for i in range(sqrtn):
            last_two = dp_in_block(blocks[i], 0)
            last_two_when_skip_first = dp_in_block(blocks[i], 1)
            cache.append((last_two, last_two_when_skip_first))
        
        ans = 0
        for (index, num) in queries:
            block_i = index // sqrtn
            blocks[block_i][index%sqrtn] = num

            last_two = dp_in_block(blocks[block_i], 0)
            last_two_when_skip_first = dp_in_block(blocks[block_i], 1)
            cache[block_i] = (last_two, last_two_when_skip_first)

            if len(cache) == 1:
                ans = (ans + max(cache[0][0])) % MOD
            else:
                dp0, dp1 = cache[0][0]
                for block_i in range(1, len(cache)):
                    dp0tmp = max(dp1 + cache[block_i][SKIP_FIRST][SECOND_LAST], dp0 + cache[block_i][USE_FIRST][SECOND_LAST])
                    dp1tmp = max(dp1 + cache[block_i][SKIP_FIRST][LAST], dp0 + cache[block_i][USE_FIRST][LAST])
                    dp0, dp1 = dp0tmp, dp1tmp
                
                ans = (ans + max(dp0, dp1)) % MOD
        
        return ans % MOD