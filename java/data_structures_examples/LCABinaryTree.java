package data_structures_examples;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class LCABinaryTree {

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

    // https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/description/
    static class Solution {
        public TreeNode lowestCommonAncestorRecursive(TreeNode root, TreeNode p, TreeNode q) {
            if (root == null) {
                return null;
            }
            if (root.val == p.val || root.val == q.val) {
                return root;
            }

            var left = lowestCommonAncestorRecursive(root.left, p, q);
            var right = lowestCommonAncestorRecursive(root.right, p, q);

            if (left != null && right != null) {
                return root;
            }

            if (left != null) {
                return left;
            } else {
                return right;
            }
        }

        public TreeNode lowestCommonAncestorIterative(TreeNode root, TreeNode p, TreeNode q) {
            Deque<TreeNode> stack = new ArrayDeque<>();
            Map<TreeNode, TreeNode> parent = new HashMap<>();

            parent.put(root, null);
            stack.push(root);

            while (!parent.containsKey(p) || !parent.containsKey(q)) {
                TreeNode node = stack.pop();

                if (node.left != null) {
                    parent.put(node.left, node);
                    stack.push(node.left);
                }
                if (node.right != null) {
                    parent.put(node.right, node);
                    stack.push(node.right);
                }
            }

            Set<TreeNode> ancestors = new HashSet<>();
            while (p != null) {
                ancestors.add(p);
                p = parent.get(p);
            }

            while (!ancestors.contains(q)) {
                q = parent.get(q);
            }

            return q;
        }
    }
}
