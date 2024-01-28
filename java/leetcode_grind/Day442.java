package leetcode_grind;

public class Day442 {
    // https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
    static class Solution1 {
        public int numSubmatrixSumTarget(int[][] matrix, int target) {
            int r = matrix.length;
            int c = matrix[0].length;

            int[][] ps = new int[r + 1][c + 1];
            for (int i = 1; i < r + 1; i++) {
                for (int j = 1; j < c + 1; j++) {
                    ps[i][j] = ps[i - 1][j] + ps[i][j - 1] - ps[i - 1][j - 1] + matrix[i - 1][j - 1];
                }
            }

            int count = 0, currSum;
            Map<Integer, Integer> h = new HashMap<>();

            for (int r1 = 1; r1 < r + 1; r1++) {
                for (int r2 = r1; r2 < r + 1; r2++) {
                    h.clear();
                    h.put(0, 1);

                    for (int col = 1; col < c + 1; ++col) {
                        currSum = ps[r2][col] - ps[r1 - 1][col];

                        count += h.getOrDefault(currSum - target, 0);
                        h.put(currSum, h.getOrDefault(currSum, 0) + 1);
                    }
                }
            }

            return count;
        }
    }

    // https://leetcode.com/problems/disconnect-path-in-a-binary-matrix-by-at-most-one-flip/
    static class Solution2 {
        public boolean isPossibleToCutPath(int[][] grid) {
            var m = grid.length;
            var n = grid[0].length;

            var dfs = new Object() {
                int apply(int x, int y) {
                    if (x >= m || y >= n) {
                        return 0;
                    }

                    if (grid[x][y] == 0) {
                        return 0;
                    }

                    if (x == m - 1 && y == n - 1) {
                        return 1;
                    }

                    if (x > 0 || y > 0) {
                        grid[x][y] = 0;
                    }

                    int cnt = 0;
                    cnt += apply(x + 1, y);
                    if (cnt == 0)
                        cnt += apply(x, y + 1);
                    return cnt > 0 ? 1 : 0;
                }
            };
            dfs.apply(0, 0);
            return dfs.apply(0, 0) == 0;
        }
    }
}
