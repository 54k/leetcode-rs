package leetcode_grind;

import java.util.ArrayDeque;

public class Day587 {
    // https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/description/
    static class Solution {
        public int minKBitFlips(int[] nums, int k) {
            var deq = new ArrayDeque<Integer>();
            var flips = 0;
            for (int index = 0; index < nums.length; index++) {
                if (!deq.isEmpty() && deq.peekFirst() < index)
                    deq.removeFirst();

                if ((nums[index] + deq.size()) % 2 == 0) {
                    if (index + k - 1 >= nums.length)
                        return -1;
                    deq.addLast(index + k - 1);
                    flips++;
                }
            }
            return flips;
        }
    }

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

    static class Solution2 {
        int sum = 0;

        public TreeNode bstToGst(TreeNode root) {
            if (root == null) {
                return null;
            }
            bstToGst(root.right);
            var t = root.val;
            root.val += sum;
            sum += t;
            bstToGst(root.left);
            return root;
        }
    }
}
