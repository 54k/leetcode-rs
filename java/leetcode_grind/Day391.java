package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;
import java.util.Stack;

public class Day391 {
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

    // https://leetcode.com/problems/construct-string-from-binary-tree/descriptions
    static class Solution1 {
        public String tree2strRecursive(TreeNode root) {
            if (root == null) {
                return "";
            }
            String res = "";
            res += root.val;
            if (root.left == null && root.right == null) {
                return res;
            }

            res += "(";
            res += tree2strRecursive(root.left);
            res += ")";

            if (root.right != null) {
                res += "(";
                res += tree2strRecursive(root.right);
                res += ")";
            }
            return res;
        }

        public String tree2strIterative(TreeNode t) {
            if (t == null) {
                return "";
            }

            Stack<TreeNode> stack = new Stack<>();
            stack.push(t);

            Set<TreeNode> visited = new HashSet<>();
            StringBuilder s = new StringBuilder();

            while (!stack.isEmpty()) {
                t = stack.peek();
                if (visited.contains(t)) {
                    stack.pop();
                    s.append(")");
                } else {
                    visited.add(t);
                    s.append("(" + t.val);
                    if (t.left == null && t.right != null) {
                        s.append("()");
                    }

                    if (t.right != null) {
                        stack.push(t.right);
                    }
                    if (t.left != null) {
                        stack.push(t.left);
                    }
                }
            }

            return s.substring(1, s.length() - 1);
        }
    }

    // https://leetcode.com/problems/reverse-pairs/
    static class Solution2 {
        public int reversePairs(int[] nums) {
            if (nums.length <= 1) {
                return 0;
            }

            int[] leftArray = Arrays.copyOfRange(nums, 0, nums.length / 2);
            int[] rightArray = Arrays.copyOfRange(nums, nums.length / 2, nums.length);

            int left = reversePairs(leftArray);
            int right = reversePairs(rightArray);

            Arrays.sort(leftArray);
            Arrays.sort(rightArray);

            int cross = countCrossInversions(leftArray, rightArray);

            return left + right + cross;
        }

        int countCrossInversions(int[] num1, int[] num2) {
            int count = 0;
            int i = 0, j = 0;
            while (i < num1.length && j < num2.length) {
                if (isReverse(num1[i], num2[j])) {
                    count += (num1.length - i);
                    j++;
                } else {
                    i++;
                }
            }
            return count;
        }

        boolean isReverse(int a, int b) {
            return (a % 2 == 0) ? a / 2 > b : (a - 1) / 2 >= b;
        }
    }
}
