package leetcode_grind;

import java.util.Arrays;
import java.util.Stack;

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

    static class Solution5 {
        boolean checkNodesValues(TreeNode node1, TreeNode node2) {
            if (node1 == null && node2 == null)
                return true;
            if (node1 != null && node2 != null && node1.val == node2.val)
                return true;
            return false;
        }

        public boolean flipEquiv(TreeNode root1, TreeNode root2) {
            Stack<TreeNode[]> nodePairStack = new Stack<>();
            nodePairStack.push(new TreeNode[] { root1, root2 });

            while (!nodePairStack.isEmpty()) {
                TreeNode[] current = nodePairStack.pop();
                TreeNode node1 = current[0];
                TreeNode node2 = current[1];

                if (node1 == null && node2 == null)
                    continue;
                if (node1 == null || node2 == null)
                    return false;
                if (node1.val != node2.val)
                    return false;

                if (checkNodesValues(node1.left, node2.left) && checkNodesValues(node1.right, node2.right)) {
                    nodePairStack.push(new TreeNode[] { node1.left, node2.left });
                    nodePairStack.push(new TreeNode[] { node1.right, node2.right });
                } else if (checkNodesValues(node1.left, node2.right) && checkNodesValues(node1.right, node2.left)) {
                    nodePairStack.push(new TreeNode[] { node1.left, node2.right });
                    nodePairStack.push(new TreeNode[] { node1.right, node2.left });
                } else {
                    return false;
                }
            }
            return true;
        }
    }

    static class Solution6 {
        void findCanonicalForm(TreeNode root) {
            if (root == null)
                return;
            findCanonicalForm(root.left);
            findCanonicalForm(root.right);

            if (root.right == null)
                return;

            if (root.left == null) {
                root.left = root.right;
                root.right = null;
                return;
            }

            TreeNode left = root.left;
            TreeNode right = root.right;
            if (left.val > right.val) {
                root.left = right;
                root.right = left;
            }
        }

        boolean areEquivalent(TreeNode root1, TreeNode root2) {
            if (root1 == null && root2 == null)
                return true;
            if (root1 == null || root2 == null)
                return false;
            if (root1.val != root2.val)
                return false;
            return (areEquivalent(root1.left, root2.left) && areEquivalent(root1.right, root2.right));
        }

        public boolean flipEquiv(TreeNode root1, TreeNode root2) {
            findCanonicalForm(root1);
            findCanonicalForm(root2);
            return areEquivalent(root1, root2);
        }
    }
}
