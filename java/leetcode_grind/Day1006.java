package leetcode_grind;

import java.util.Stack;

public class Day1006 {

    // https://leetcode.com/problems/count-submatrices-with-all-ones/description/?envType=daily-question&envId=2025-08-21
    static class Solution1 {
        public int numSubmat(int[][] mat) {
            int m = mat.length;
            int n = mat[0].length;
            int res = 0;
            int[][] row = new int[m][n];

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (j == 0) {
                        row[i][j] = mat[i][j];
                    } else {
                        row[i][j] = mat[i][j] == 0 ? 0 : row[i][j - 1] + 1;
                    }
                    int cur = row[i][j];
                    for (int k = i; k >= 0; k--) {
                        cur = Math.min(cur, row[k][j]);
                        if (cur == 0) {
                            break;
                        }
                        res += cur;
                    }
                }
            }
            return res;
        }
    }

    static class Solution2 {
        public int numSubmat(int[][] mat) {
            int n = mat[0].length;
            int[] heights = new int[n];
            int res = 0;
            for (int[] row : mat) {
                for (int i = 0; i < n; i++) {
                    heights[i] = row[i] == 0 ? 0 : heights[i] + 1;
                }

                Stack<int[]> stack = new Stack<>();
                stack.push(new int[] { -1, 0, -1 });

                for (int i = 0; i < n; i++) {
                    int h = heights[i];
                    while (stack.peek()[2] >= h) {
                        stack.pop();
                    }
                    int[] top = stack.peek();
                    int j = top[0];
                    int prev = top[1];
                    int cur = prev + (i - j) * h;
                    stack.push(new int[] { i, cur, h });
                    res += cur;
                }
            }
            return res;
        }
    }

}
