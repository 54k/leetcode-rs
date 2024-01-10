package leetcode_grind;

public class Day424 {
    // https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/description
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

    static class Solution1 {
        int maxDistance = 0;

        public int amountOfTime(TreeNode root, int start) {
            traverse(root, start);
            return maxDistance;
        }

        int traverse(TreeNode root, int start) {
            int depth = 0;
            if (root == null) {
                return depth;
            }

            int leftDepth = traverse(root.left, start); // 2
            int rightDepth = traverse(root.right, start); // -1

            if (root.val == start) {
                maxDistance = Math.max(leftDepth, rightDepth);
                depth = -1;
            } else if (leftDepth >= 0 && rightDepth >= 0) {
                depth = Math.max(leftDepth, rightDepth) + 1;
            } else {
                int distance = Math.abs(leftDepth) + Math.abs(rightDepth);
                maxDistance = Math.max(maxDistance, distance);
                depth = Math.min(leftDepth, rightDepth) - 1;
            }

            return depth;
        }
    }
}
