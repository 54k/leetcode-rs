package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
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

    static class Solution3 {
        public List<Integer> boundaryOfBinaryTree(TreeNode root) {
            List<Integer> left_boundary = new LinkedList<>();
            List<Integer> right_boundary = new LinkedList<>();
            List<Integer> leaves = new LinkedList<>();
            preorder(root, left_boundary, right_boundary, leaves, 0);
            left_boundary.addAll(leaves);
            left_boundary.addAll(right_boundary);
            return left_boundary;
        }

        boolean isLeaf(TreeNode cur) {
            return (cur.left == null && cur.right == null);
        }

        boolean isRightBoundary(int flag) {
            return flag == 2;
        }

        boolean isLeftBoundary(int flag) {
            return flag == 1;
        }

        boolean isRoot(int flag) {
            return flag == 0;
        }

        int leftChildFlag(TreeNode cur, int flag) {
            if (isLeftBoundary(flag) || isRoot(flag)) {
                return 1;
            } else if (isRightBoundary(flag) && cur.right == null) {
                return 2;
            }
            return 3;
        }

        int rightChildFlag(TreeNode cur, int flag) {
            if (isRightBoundary(flag) || isRoot(flag)) {
                return 2;
            } else if (isLeftBoundary(flag) && cur.left == null) {
                return 1;
            }
            return 3;
        }

        void preorder(TreeNode cur, List<Integer> left_boundary, List<Integer> right_boundary, List<Integer> leaves,
                int flag) {
            if (cur == null) {
                return;
            }
            if (isRightBoundary(flag)) {
                right_boundary.add(0, cur.val);
            } else if (isLeftBoundary(flag) || isRoot(flag)) {
                left_boundary.add(cur.val);
            } else if (isLeaf(cur)) {
                leaves.add(cur.val);
            }
            preorder(cur.left, left_boundary, right_boundary, leaves, leftChildFlag(cur, flag));
            preorder(cur.right, left_boundary, right_boundary, leaves, rightChildFlag(cur, flag));
        }
    }
}