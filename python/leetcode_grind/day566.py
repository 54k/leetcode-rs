# https://leetcode.com/problems/is-subsequence/description/
class Solution1:
    def isSubsequence(self, s: str, t: str) -> bool:
        if not len(s):
            return True

        i, j = 0, 0
        while i < len(s) and j < len(t):
            if s[i] == t[j]:
                i+=1
            j+=1
        return i == len(s)