package leetcode_grind;

public class Day550 {

    public class TreeNode {
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

    // https://leetcode.com/problems/delete-leaves-with-a-given-value/description
    static class Solution1 {
        public TreeNode removeLeafNodes(TreeNode root, int target) {
            if (root == null) {
                return root;
            }
            if (root.left == null && root.right == null) {
                return root.val == target ? null : root;
            }
            root.left = removeLeafNodes(root.left, target);
            root.right = removeLeafNodes(root.right, target);
            if (root.left == null && root.right == null) {
                return root.val == target ? null : root;
            }
            return root;
        }
    }
}