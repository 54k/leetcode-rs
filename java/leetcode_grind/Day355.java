package leetcode_grind;

public class Day355 {
    static public class TreeNode {
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

    // https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/description
    static class Solution1 {
        public int averageOfSubtree(TreeNode root) {
            var ans = new int[] { 0 };
            var preorder = new Object() {
                int[] apply(TreeNode node) {
                    if (node == null) {
                        return new int[] { 0, 0 };
                    }

                    int[] left = apply(node.left);
                    int[] right = apply(node.right);

                    var sum = left[0] + right[0] + node.val;
                    var count = left[1] + right[1] + 1;
                    if (node.val == sum / count) {
                        ans[0]++;
                    }
                    return new int[] { sum, count };
                }
            };
            preorder.apply(root);
            return ans[0];
        }
    }

    // https://leetcode.com/problems/minimum-falling-path-sum/description/
    static class Solution2 {
        public int minFallingPathSum(int[][] matrix) {
            var n = matrix[0].length;
            for (int i = matrix.length - 2; i >= 0; i--) {
                for (int j = 0; j < n; j++) {
                    matrix[i][j] += Math.min(
                            matrix[i + 1][j],
                            Math.min(matrix[i + 1][Math.max(j - 1, 0)], matrix[i + 1][Math.min(j + 1, n - 1)]));
                }
            }

            var ans = Integer.MAX_VALUE;
            for (int i = 0; i < n; i++) {
                ans = Math.min(ans, matrix[0][i]);
            }
            return ans;
        }
    }
}
