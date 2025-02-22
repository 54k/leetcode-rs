package leetcode_grind;

import java.util.*;

public class Day826 {
    // https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/description/?envType=daily-question&envId=2025-02-22
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
        static int index = 0;

        public TreeNode recoverFromPreorder(String traversal) {
            index = 0;
            return helper(traversal, 0);
        }

        TreeNode helper(String traversal, int depth) {
            if (index >= traversal.length())
                return null;
            int dashCount = 0;
            while ((index + dashCount) < traversal.length() && traversal.charAt(index + dashCount) == '-') {
                dashCount++;
            }
            if (dashCount != depth)
                return null;
            index += dashCount;

            int value = 0;
            while (index < traversal.length() && Character.isDigit(traversal.charAt(index))) {
                value = value * 10 + (traversal.charAt(index++) - '0');
            }

            TreeNode node = new TreeNode(value);
            node.left = helper(traversal, depth + 1);
            node.right = helper(traversal, depth + 1);
            return node;
        }
    }
}
