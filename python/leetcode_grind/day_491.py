# https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/description/
class Solution1:
    def findMinArrowShots(self, points: List[List[int]]) -> int:
        points = sorted(points, key = lambda x:x[1])
        ans = 1
        prev = points[0][1]
        for i in range(1, len(points)):
            if points[i][0] <= prev:
                continue
            else:
                prev = points[i][1]
                ans+=1
        return ans

# https://leetcode.com/problems/count-vowels-permutation/description/
class Solution2:
    def countVowelPermutation(self, n: int) -> int:
        a = e = i = o = u = 1
        mod = 10**9+7
        for _ in range(2, n+1):
            a, e, i, o, u = (e + u + i) % mod, (a + i) % mod, (o + e) % mod, i, (i + o) % mod
        return sum([a,e,i,o,u]) % mod