package leetcode_grind;

import java.util.Stack;

public class Day470 {
    // https://leetcode.com/problems/same-tree/description
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
        public boolean isSameTree(TreeNode p, TreeNode q) {
            var stack = new Stack<TreeNode>();
            stack.push(p);
            stack.push(q);
            while (stack.size() >= 2) {
                var a = stack.pop();
                var b = stack.pop();

                if ((a == null && b != null) || (a != null && b == null)
                        || (a != null && b != null && a.val != b.val)) {
                    return false;
                }

                if (a == null && b == null) {
                    continue;
                }

                if (a != null) {
                    stack.push(a.left);
                } else {
                    stack.push(null);
                }

                if (b != null) {
                    stack.push(b.left);
                } else {
                    stack.push(null);
                }

                if (a != null) {
                    stack.push(a.right);
                } else {
                    stack.push(null);
                }

                if (b != null) {
                    stack.push(b.right);
                } else {
                    stack.push(null);
                }
            }

            return stack.isEmpty();
        }
    }

}
