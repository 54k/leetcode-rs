package leetcode_grind;

import java.util.Stack;

public class Day422 {
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

    // https://leetcode.com/problems/range-sum-of-bst/description
    static class Solution1 {
        public int rangeSumBST(TreeNode root, int low, int high) {
            int ans = 0;
            Stack<TreeNode> stack = new Stack<>();
            stack.push(root);

            while (!stack.isEmpty()) {
                TreeNode node = stack.pop();
                if (node != null) {
                    if (low <= node.val && node.val <= high) {
                        ans += node.val;
                    }
                    if (low < node.val) {
                        stack.push(node.left);
                    }
                    if (high > node.val) {
                        stack.push(node.right);
                    }
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/subtree-of-another-tree/description/
    static class Solution2 {
        boolean isSame(TreeNode root, TreeNode subRoot) {
            if (root == null && subRoot == null) {
                return true;
            } else if (root == null && subRoot != null) {
                return false;
            } else if (root != null && subRoot == null) {
                return false;
            }
            return root.val == subRoot.val && isSame(root.left, subRoot.left) && isSame(root.right, subRoot.right);
        }

        public boolean isSubtree(TreeNode root, TreeNode subRoot) {
            if (root == null) {
                return subRoot == null;
            }

            if (root.val == subRoot.val) {
                return isSame(root, subRoot) || isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
            }

            return isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
        }
    }
}
