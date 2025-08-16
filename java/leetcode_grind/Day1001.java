package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day1001 {
    // https://leetcode.com/problems/maximum-69-number/description/?envType=daily-question&envId=2025-08-16
    static class Solution1 {
        public int maximum69Number(int num) {
            int numCopy = num;
            int indexSix = -1;
            int currDigit = 0;

            while (numCopy > 0) {
                if (numCopy % 10 == 6) {
                    indexSix = currDigit;
                }
                numCopy /= 10;
                currDigit++;
            }

            return indexSix == -1 ? num : num + 3 * (int) Math.pow(10, indexSix);
        }
    }

    // https://leetcode.com/problems/boundary-of-binary-tree/description/

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

    static class Solution2 {
        boolean isLeaf(TreeNode t) {
            return t.left == null && t.right == null;
        }

        void addLeaves(List<Integer> res, TreeNode root) {
            if (isLeaf(root)) {
                res.add(root.val);
            } else {
                if (root.left != null) {
                    addLeaves(res, root.left);
                }
                if (root.right != null) {
                    addLeaves(res, root.right);
                }
            }
        }

        public List<Integer> boundaryOfBinaryTree(TreeNode root) {
            List<Integer> res = new ArrayList<>();
            if (root == null) {
                return res;
            }
            if (!isLeaf(root)) {
                res.add(root.val);
            }
            TreeNode t = root.left;
            while (t != null) {
                if (!isLeaf(t)) {
                    res.add(t.val);
                }
                if (t.left != null) {
                    t = t.left;
                } else {
                    t = t.right;
                }
            }
            addLeaves(res, root);
            Stack<Integer> s = new Stack<>();
            t = root.right;
            while (t != null) {
                if (!isLeaf(t)) {
                    s.push(t.val);
                }
                if (t.right != null) {
                    t = t.right;
                } else {
                    t = t.left;
                }
            }
            while (!s.empty()) {
                res.add(s.pop());
            }
            return res;
        }
    }
}