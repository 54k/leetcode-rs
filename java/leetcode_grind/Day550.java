package leetcode_grind;

import java.util.Stack;

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

    static class Solution2 {
        public TreeNode removeLeafNodes(TreeNode root, int target) {
            Stack<TreeNode> stack = new Stack<>();
            TreeNode currentNode = root, lastRightNode = null;

            while (!stack.isEmpty() || currentNode != null) {
                while (currentNode != null) {
                    stack.push(currentNode);
                    currentNode = currentNode.left;
                }

                currentNode = stack.peek();

                if (currentNode.right != lastRightNode && currentNode.right != null) {
                    currentNode = currentNode.right;
                    continue;
                }

                stack.pop();

                if (currentNode.right == null && currentNode.left == null && currentNode.val == target) {
                    if (stack.isEmpty()) {
                        return null;
                    }

                    TreeNode parent = stack.isEmpty() ? null : stack.peek();

                    if (parent != null) {
                        if (parent.left == currentNode) {
                            parent.left = null;
                        } else {
                            parent.right = null;
                        }
                    }
                }

                lastRightNode = currentNode;
                currentNode = null;
            }
            return root;
        }
    }
}