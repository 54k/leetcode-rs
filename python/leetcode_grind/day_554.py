# https://leetcode.com/problems/longest-palindromic-substring/description/
class Solution1:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        dp = [[False] * n for _ in range(n)]
        ans = [0, 0]

        for i in range(n):
            dp[i][i] = True
        
        for i in range(n - 1):
            if s[i] == s[i+1]:
                dp[i][i+1] = True
                ans = [i, i+1]
        
        for diff in range(2, n):
            for i in range(n - diff):
                j = i + diff
                if s[i] == s[j] and dp[i + 1][j - 1]:
                    dp[i][j] = True
                    ans = [i, j]
        
        i, j = ans
        return s[i : j + 1]

class Solution2:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        ans = [0, 0]
        dp = [[False] * n for _ in range(n)]
        for i in range(n-1, -1, -1):
            for j in range(i, n):
                if s[i] == s[j] and (j - i <= 2 or dp[i+1][j-1]):
                    dp[i][j] = True
                    if j - i >= ans[1] - ans[0]:
                        ans = [i, j]
        return s[ans[0]:ans[1]+1]