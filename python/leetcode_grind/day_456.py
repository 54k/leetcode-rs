# https://leetcode.com/problems/palindromic-substrings/description/?envType=daily-question&envId=2024-02-10
class Solution:
    def countSubstrings(self, s: str) -> int:
        n = len(s)
        dp = [[False] * n for _ in range(n)]
        ans = 0

        for i in range(n):
            dp[i][i] = True
            ans += 1

        for L in range(2, n+1):
            for left in range(n-L+1):
                right = left + L - 1
                
                if s[left] == s[right] and (L == 2 or dp[left+1][right-1]):
                    dp[left][right] = True
                    ans+=1

        return ans