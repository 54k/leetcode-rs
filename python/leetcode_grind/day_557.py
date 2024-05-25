# https://leetcode.com/problems/word-break-ii/description
class Solution1:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        n = len(s)
        wd = set(wordDict)
        ans = []
        def dfs(i, cur):
            # print(f"{i} of {n}, {cur}")
            if i == n:
                ans.append(" ".join(cur))
                return
            for k in range(i, n+1):
                ss = s[i:k]
                if ss in wd:
                    dfs(k, cur + [ss])

        dfs(0, [])
        return ans