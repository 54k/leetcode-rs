# https://leetcode.com/problems/path-with-maximum-gold/description
class Solution1:
    def getMaximumGold(self, grid: List[List[int]]) -> int:
        DIRECTIONS = [0, 1, 0, -1, 0]
        rows = len(grid)
        cols = len(grid[0])

        def bfs_backtrack(row, col):
            queue = deque()
            visited = set()
            max_gold = 0
            visited.add((row, col))
            queue.append((row, col, grid[row][col], visited))

            while queue:
                curr_row, curr_col, curr_gold, curr_vis = queue.popleft()
                max_gold = max(max_gold, curr_gold)

                for direction in range(4):
                    next_row = curr_row + DIRECTIONS[direction]
                    next_col = curr_col + DIRECTIONS[direction + 1]

                    if  0 <= next_row < rows and 0 <= next_col < cols and \
                            grid[next_row][next_col] != 0 and \
                            (next_row, next_col) not in curr_vis:
                        curr_vis.add((next_row, next_col))
                        queue.append((next_row, next_col, curr_gold + grid[next_row][next_col], curr_vis.copy()))
                        curr_vis.remove((next_row, next_col))

            return max_gold

        total_gold = sum(sum(row) for row in grid) 
        max_gold = 0
        for row in range(rows):
            for col in range(cols):
                if grid[row][col] != 0:
                    max_gold = max(max_gold, bfs_backtrack(row, col))
                    if max_gold == total_gold:
                        return total_gold
        return max_gold

# https://leetcode.com/problems/maximum-difference-score-in-a-grid/description/
class Solution2:
    def maxScore(self, A: List[List[int]]) -> int:
        res, m, n = -inf, len(A), len(A[0])
        for i in range(m):
            for j in range(n):
                pre = min(A[i-1][j] if i else inf, A[i][j-1] if j else inf)
                res = max(res, A[i][j] - pre)
                A[i][j] = min(A[i][j], pre)
        return res
    
# https://leetcode.com/problems/super-egg-drop/description/
class Solution3:
    def superEggDrop(self, k: int, n: int) -> int:
        @lru_cache(None)
        def dp(k, n):
            if n == 0:
                return 0
            if k == 1:
                return n
            lo, hi = 1, n
            while lo + 1 < hi:
                x = (lo + hi) // 2
                t1 = dp(k-1, x-1)
                t2 = dp(k, n-x)

                if t1 < t2:
                    lo = x
                elif t1 > t2:
                    hi = x
                else:
                    lo = hi = x
            return 1 + min(max(dp(k-1, x-1), dp(k, n-x)) for x in (lo, hi))
        return dp(k, n)