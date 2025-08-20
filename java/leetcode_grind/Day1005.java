package leetcode_grind;

import java.util.Arrays;

public class Day1005 {
    // https://leetcode.com/problems/count-square-submatrices-with-all-ones/description/?envType=daily-question&envId=2025-08-20
    static class Solution1 {
        public int countSquares(int[][] matrix) {
            int row = matrix.length, col = matrix[0].length;
            int[][] dp = new int[row + 1][col + 1];
            int ans = 0;

            for (int i = 0; i < row; i++) {
                for (int j = 0; j < col; j++) {
                    if (matrix[i][j] == 1) {
                        dp[i + 1][j + 1] = Math.min(Math.min(dp[i][j + 1], dp[i + 1][j]), dp[i][j]) + 1;
                        ans += dp[i + 1][j + 1];
                    }
                }
            }

            return ans;
        }
    }

    static class Solution2 {
        int solve(int i, int j, int[][] grid, int[][] dp) {
            if (i >= grid.length || j >= grid[0].length) {
                return 0;
            }
            if (grid[i][j] == 0) {
                return 0;
            }
            if (dp[i][j] != -1) {
                return dp[i][j];
            }

            int right = solve(i, j + 1, grid, dp);
            int diagonal = solve(i + 1, j + 1, grid, dp);
            int below = solve(i + 1, j, grid, dp);

            return dp[i][j] = 1 + Math.min(right, Math.min(diagonal, below));
        }

        public int countSquares(int[][] grid) {
            int ans = 0;
            int[][] dp = new int[grid.length][grid[0].length];
            for (int i = 0; i < grid.length; i++) {
                for (int j = 0; j < grid[0].length; j++) {
                    Arrays.fill(dp[i], -1);
                }
            }
            for (int i = 0; i < grid.length; i++) {
                for (int j = 0; j < grid[0].length; j++) {
                    ans += solve(i, j, grid, dp);
                }
            }
            return ans;
        }
    }

    static class Solution3 {
        public int countSquares(int[][] matrix) {
            int row = matrix.length, col = matrix[0].length, result = 0, prev = 0;
            int[] dp = new int[col + 1];

            for (int i = 1; i <= row; i++) {
                for (int j = 1; j <= col; j++) {
                    if (matrix[i - 1][j - 1] == 1) {
                        int temp = dp[j];
                        dp[j] = 1 + Math.min(prev, Math.min(dp[j - 1], dp[j]));
                        prev = temp;
                        result += dp[j];
                    } else {
                        dp[j] = 0;
                    }
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/flip-equivalent-binary-trees/description/
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    static class Solution4 {
        public boolean flipEquiv(TreeNode root1, TreeNode root2) {
            if (root1 == null && root2 == null) {
                return true;
            }
            if (root1 == null || root2 == null) {
                return false;
            }
            if (root1.val != root2.val) {
                return false;
            }

            boolean noSwap = flipEquiv(root1.left, root2.left) &&
                    flipEquiv(root1.right, root2.right);
            boolean swap = flipEquiv(root1.left, root2.right) &&
                    flipEquiv(root1.right, root2.left);
            return noSwap || swap;
        }
    }
}
