# https://leetcode.com/problems/score-of-a-string/description
class Solution1:
    def scoreOfString(self, s: str) -> int:
        ans = 0
        for i in range(1, len(s)):
            ans += abs(ord(s[i-1])-ord(s[i]))
        return ans