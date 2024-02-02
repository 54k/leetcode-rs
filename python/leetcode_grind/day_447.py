import math

# https://leetcode.com/problems/sequential-digits/
class Solution1:
    def sequentialDigits(self, low: int, high: int) -> List[int]:
        sample = "123456789"
        nums = []

        for l in range(2, 10):
            for start in range(0, 10 - l):
                nums.append(int(sample[start:start+l]))

        return [x for x in nums if low <= x <= high]

# https://leetcode.com/problems/perfect-squares/
class Solution:
    def numSquares(self, n: int) -> int:
        msi = int(math.sqrt(n))+1 # max square idx
        square_nums = [i**2 for i in range(1, msi)]

        dp = [1000000007] * (n + 1)
        dp[0] = 0

        for i in range(1, n+1):
            for s in square_nums:
                if i < s:
                    break
                dp[i] = min(dp[i], dp[i-s] + 1)

        return dp[-1]

# https://leetcode.com/problems/flip-game/
class Solution:
    def generatePossibleNextMoves(self, currentState: str) -> List[str]:
        ans = []
        for s in range(0, len(currentState) - 1):
            if "++" == currentState[s:s+2]:
                ans.append(currentState[:s] + "--" + currentState[s+2:])
    
        return ans
