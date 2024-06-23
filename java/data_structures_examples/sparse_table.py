# https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/description/
class Solution:
    def longestSubarray(self, nums: List[int], limit: int) -> int:
        n = len(nums)

        mn = [[0] * n for _ in range(int(math.log2(n)) + 1)]
        mx = [[0] * n for _ in range(int(math.log2(n)) + 1)]

        for i in range(int(math.log2(n)) + 1):
            for j in range(n - (1 << i) + 1):
                if not i:
                    mn[i][j] = nums[j]
                    mx[i][j] = nums[j]
                else:
                    mn[i][j] = min(mn[i-1][j], mn[i-1][j + (1 << (i - 1))])
                    mx[i][j] = max(mx[i-1][j], mx[i-1][j + (1 << (i - 1))])

        def get(l, r, min_or_max, st):
            ll = r - l + 1
            lg2 = int(math.log2(ll))
            return min_or_max(st[lg2][l], st[lg2][r - (1 << lg2) + 1])

        ans = 0
        s = 0
        for e in range(n):
            while abs(get(s, e, max, mx) - get(s, e, min, mn)) > limit:
                s += 1
            ans = max(ans, e - s + 1)
        return ans