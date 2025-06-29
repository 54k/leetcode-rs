package leetcode_grind;

import java.util.Arrays;

public class Day952 {
    // https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/description/?envType=daily-question&envId=2025-06-28
    static class Solution1 {
        public int[] maxSubsequence(int[] nums, int k) {
            int n = nums.length;
            int[][] vals = new int[n][2];
            for (int i = 0; i < n; i++) {
                vals[i][0] = i;
                vals[i][1] = nums[i];
            }
            Arrays.sort(vals, (x1, x2) -> Integer.compare(x2[1], x1[1]));
            Arrays.sort(vals, 0, k, (x1, x2) -> Integer.compare(x1[0], x2[0]));
            int[] res = new int[k];
            for (int i = 0; i < k; i++) {
                res[i] = vals[i][1];
            }
            return res;
        }
    }

    static class Solution2 {
        public int numSubseq(int[] nums, int target) {
            int n = nums.length;
            int mod = 1_000_000_007;
            Arrays.sort(nums);

            int[] power = new int[n];
            power[0] = 1;
            for (int i = 1; i < n; i++) {
                power[i] = (power[i - 1] * 2) % mod;
            }

            int answer = 0;
            int left = 0, right = n - 1;

            while (left <= right) {
                if (nums[left] + nums[right] <= target) {
                    answer += power[right - left];
                    answer %= mod;
                    left++;
                } else {
                    right--;
                }
            }

            return answer;
        }
    }

    static class Node {
        public int val;
        public Node left;
        public Node right;

        public Node() {
        }

        public Node(int _val) {
            val = _val;
        }

        public Node(int _val, Node _left, Node _right) {
            val = _val;
            left = _left;
            right = _right;
        }
    };

    static class Solution3 {
        Node first = null;
        Node last = null;

        void helper(Node node) {
            if (node != null) {
                helper(node.left);
                if (last != null) {
                    last.right = node;
                    node.left = last;
                } else {
                    first = node;
                }
                last = node;
                helper(node.right);
            }
        }

        public Node treeToDoublyList(Node root) {
            if (root == null)
                return null;
            helper(root);
            last.right = first;
            first.left = last;
            return first;
        }
    }
}
