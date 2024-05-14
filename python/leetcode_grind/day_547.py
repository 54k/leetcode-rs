# https://leetcode.com/problems/path-with-maximum-gold/description
class Solution:
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