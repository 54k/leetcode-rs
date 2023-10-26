package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Day348 {
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

    // https://leetcode.com/problems/binary-tree-inorder-traversal/description/
    static class Solution {
        public List<Integer> inorderTraversalIterative(TreeNode root) {
            var result = new ArrayList<Integer>();
            var curr = root;
            var stack = new LinkedList<TreeNode>();
            while (curr != null || !stack.isEmpty()) {
                while (curr.left != null) {
                    stack.push(curr);
                    curr = curr.left;
                }

                curr = stack.pop();
                result.add(curr.val);
                curr = curr.right;
            }
            return result;
        }

        public List<Integer> inorderTraversalMorris1(TreeNode root) {
            var result = new ArrayList<Integer>();
            var curr = root;
            while (curr != null) {
                if (curr.left == null) {
                    result.add(curr.val);
                    curr = curr.right;
                } else {
                    var rightmost = curr.left;
                    while (rightmost.right != null && rightmost.right != curr) {
                        rightmost = rightmost.right;
                    }

                    if (rightmost.right == null) {
                        rightmost.right = curr;
                        curr = curr.left;
                    } else {
                        result.add(curr.val);
                        curr = curr.right;
                        rightmost.right = null;
                    }
                }
            }
            return result;
        }

        public List<Integer> inorderTraversalMorris2(TreeNode root) {
            var result = new ArrayList<Integer>();
            var curr = root;
            while (curr != null) {
                if (curr.left == null) {
                    result.add(curr.val);
                    curr = curr.right;
                } else {
                    var rightmost = curr.left;
                    while (rightmost.right != null) {
                        rightmost = rightmost.right;
                    }
                    rightmost.right = curr;
                    var tmp = curr;
                    curr = curr.left;
                    tmp.left = null;
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/binary-tree-preorder-traversal/description/
    static class Solution2 {
        public List<Integer> preorderTraversalMorris1(TreeNode root) {
            var result = new ArrayList<Integer>();
            var curr = root;
            while (curr != null) {
                if (curr.left == null) {
                    result.add(curr.val);
                    curr = curr.right;
                } else {
                    var rightmost = curr.left;
                    while (rightmost.right != null && rightmost.right != curr) {
                        rightmost = rightmost.right;
                    }
                    if (rightmost.right == null) {
                        rightmost.right = curr;
                        result.add(curr.val);
                        curr = curr.left;
                    } else {
                        curr = curr.right;
                        rightmost.right = null;
                    }
                }
            }
            return result;
        }
    }
}
