package leetcode_grind;

import java.util.Stack;

public class Day939 {
    // https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/description/?envType=daily-question&envId=2025-06-15
    static class Solution1 {
        public int maxDiff(int num) {
            int min_num = num;
            int max_num = num;
            for (int x = 0; x < 10; x++) {
                for (int y = 0; y < 10; y++) {
                    String res = change(num, x, y);
                    if (res.charAt(0) != '0') {
                        int res_i = Integer.parseInt(res);
                        min_num = Math.min(min_num, res_i);
                        max_num = Math.max(max_num, res_i);
                    }
                }
            }
            return max_num - min_num;
        }

        String change(int num, int x, int y) {
            StringBuffer num_s = new StringBuffer(String.valueOf(num));
            int length = num_s.length();
            for (int i = 0; i < length; i++) {
                char digit = num_s.charAt(i);
                if (digit - '0' == x) {
                    num_s.setCharAt(i, (char) ('0' + y));
                }
            }
            return num_s.toString();
        }
    }

    static class Solution2 {
        public int maxDiff(int num) {
            StringBuffer min_num = new StringBuffer(String.valueOf(num));
            StringBuffer max_num = new StringBuffer(String.valueOf(num));

            int max_length = max_num.length();
            for (int i = 0; i < max_length; i++) {
                char digit = max_num.charAt(i);
                if (digit != '9') {
                    replace(max_num, digit, '9');
                    break;
                }
            }

            int min_length = min_num.length();
            for (int i = 0; i < min_length; i++) {
                char digit = min_num.charAt(i);
                if (i == 0) {
                    if (digit != '1') {
                        replace(min_num, digit, '1');
                        break;
                    }
                } else {
                    if (digit != '0' && digit != min_num.charAt(0)) {
                        replace(min_num, digit, '0');
                        break;
                    }
                }
            }

            return (Integer.parseInt(max_num.toString()) - Integer.parseInt(min_num.toString()));
        }

        void replace(StringBuffer s, char x, char y) {
            int length = s.length();
            for (int i = 0; i < length; i++) {
                if (s.charAt(i) == x) {
                    s.setCharAt(i, y);
                }
            }
        }
    }

    // https://leetcode.com/problems/equal-tree-partition/description/?envType=weekly-question&envId=2025-06-15
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

    static class Solution3 {
        Stack<Integer> seen;

        public boolean checkEqualTree(TreeNode root) {
            seen = new Stack<>();
            int total = sum(root);
            seen.pop();
            if (total % 2 == 0) {
                for (int s : seen) {
                    if (s == total / 2) {
                        return true;
                    }
                }
            }
            return false;
        }

        int sum(TreeNode node) {
            if (node == null) {
                return 0;
            }
            seen.push(sum(node.left) + sum(node.right) + node.val);
            return seen.peek();
        }
    }
}
