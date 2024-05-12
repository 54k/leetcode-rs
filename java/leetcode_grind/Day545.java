package leetcode_grind;

public class Day545 {
    // https://leetcode.com/problems/largest-local-values-in-a-matrix/description/
    static class Solution1 {
        public int[][] largestLocal(int[][] grid) {
            var findMax = new Object() {
                int apply(int x, int y) {
                    int maxElement = 0;
                    for (int i = x; i < x + 3; i++) {
                        for (int j = y; j < y + 3; j++) {
                            maxElement = Math.max(maxElement, grid[i][j]);
                        }
                    }
                    return maxElement;
                }
            };

            int N = grid.length;
            int[][] maxLocal = new int[N - 2][N - 2];
            for (int i = 0; i < N - 2; i++) {
                for (int j = 0; j < N - 2; j++) {
                    maxLocal[i][j] = findMax.apply(i, j);
                }
            }
            return maxLocal;
        }
    }
}
