package leetcode_grind;

public class Day415 {
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

    // https://leetcode.com/problems/range-sum-of-bst/description/
    static class Solution1 {
        public int rangeSumBST(TreeNode root, int low, int high) {
            if (root == null) {
                return 0;
            }
            int sum = 0;
            if (low <= root.val && root.val <= high) {
                sum += root.val;
            }
            return sum + rangeSumBST(root.left, low, high) + rangeSumBST(root.right, low, high);
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
    static class Solution2 {
        public int maxProfit(int[] prices, int fee) {
            int free = 0;
            int hold = -prices[0];

            for (int i = 1; i < prices.length; i++) {
                free = Math.max(free, hold + prices[i] - fee);
                hold = Math.max(hold, free - prices[i]);
            }

            return Math.max(free, hold);
        }
    }
}
