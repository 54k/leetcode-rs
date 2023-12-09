package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day392 {
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

    // https://leetcode.com/problems/binary-tree-inorder-traversal/description
    static class Solution {
        public List<Integer> inorderTraversalIterative(TreeNode root) {
            var result = new ArrayList<Integer>();
            var stack = new Stack<TreeNode>();
            var curr = root;
            while (!stack.isEmpty() || curr != null) {
                while (curr != null) {
                    stack.push(curr);
                    curr = curr.left;
                }

                curr = stack.pop();
                result.add(curr.val);
                curr = curr.right;
            }
            return result;
        }

        public List<Integer> inorderTraversalMorris(TreeNode root) {
            var ans = new ArrayList<Integer>();
            var curr = root;
            while (curr != null) {
                if (curr.left == null) {
                    ans.add(curr.val);
                    curr = curr.right;
                } else {
                    var ancestor = curr.left;
                    while (ancestor.right != null) {
                        ancestor = ancestor.right;
                    }

                    ancestor.right = curr;
                    var tmp = curr;
                    curr = curr.left;
                    tmp.left = null;
                }
            }
            return ans;
        }
    }

}
