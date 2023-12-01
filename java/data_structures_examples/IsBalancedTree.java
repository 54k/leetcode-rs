package data_structures_examples;

public class IsBalancedTree {
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

    // https://leetcode.com/problems/balanced-binary-tree/description/
    static class Solution {
        public boolean isBalanced1(TreeNode root) {
            var height = new Object() {
                int apply(TreeNode root) {
                    if (root == null) {
                        return -1;
                    }
                    return 1 + Math.max(apply(root.left), apply(root.right));
                }
            };

            if (root == null) {
                return true;
            }
            return Math.abs(height.apply(root.left) - height.apply(root.right)) < 2 && isBalanced(root.left)
                    && isBalanced(root.right);
        }

        static class TreeInfo {
            int height;
            boolean isBalanced;

            TreeInfo(int h, boolean b) {
                height = h;
                isBalanced = b;
            }
        }

        public TreeInfo isBalancedHelper(TreeNode root) {
            if (root == null) {
                return new TreeInfo(-1, true);
            }

            TreeInfo left = isBalancedHelper(root.right);
            if (!left.isBalanced) {
                return new TreeInfo(-1, false);
            }
            TreeInfo right = isBalancedHelper(root.left);
            if (!right.isBalanced) {
                return new TreeInfo(-1, false);
            }

            if (Math.abs(left.height - right.height) < 2) {
                return new TreeInfo(Math.max(left.height, right.height) + 1, true);
            }
            return new TreeInfo(-1, false);
        }

        public boolean isBalanced2(TreeNode root) {
            return isBalancedHelper(root).isBalanced;
        }
    }
}
