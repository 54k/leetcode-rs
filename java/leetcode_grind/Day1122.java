package leetcode_grind;

public class Day1122 {
    // https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/description/?envType=daily-question&envId=2025-12-15
    static class Solution1 {
        public long getDescentPeriods(int[] prices) {
            long ans = 1;
            long per = 1;
            for (int i = 1; i < prices.length; i++) {
                if (prices[i - 1] - prices[i] == 1) {
                    per++;
                } else {
                    per = 1;
                }
                ans += per;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/description/?envType=weekly-question&envId=2025-12-15
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

    static class Solution2 {
        public int longestConsecutive(TreeNode root) {
            return dfs(root, null, 0);
        }

        int dfs(TreeNode p, TreeNode parent, int length) {
            if (p == null)
                return length;
            length = (parent != null && p.val == parent.val + 1) ? length + 1 : 1;
            return Math.max(length, Math.max(dfs(p.left, p, length), dfs(p.right, p, length)));
        }
    }

    static class Solution3 {
        int maxLength = 0;

        public int longestConsecutive(TreeNode root) {
            dfs(root);
            return maxLength;
        }

        int dfs(TreeNode p) {
            if (p == null)
                return 0;
            int L = dfs(p.left) + 1;
            int R = dfs(p.right) + 1;
            if (p.left != null && p.val + 1 != p.left.val) {
                L = 1;
            }
            if (p.right != null && p.val + 1 != p.right.val) {
                R = 1;
            }
            int length = Math.max(L, R);
            maxLength = Math.max(maxLength, length);
            return length;
        }
    }

    // https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/description/?envType=daily-question&envId=2025-12-14
    static class Solution4 {
        public int numberOfWays(String corridor) {
            final int MOD = 1_000_000_007;
            int[][] count = new int[corridor.length() + 1][3];

            count[corridor.length()][0] = 0;
            count[corridor.length()][1] = 0;
            count[corridor.length()][2] = 1;

            for (int index = corridor.length() - 1; index >= 0; index--) {
                if (corridor.charAt(index) == 'S') {
                    count[index][0] = count[index + 1][1];
                    count[index][1] = count[index + 1][2];
                    count[index][2] = count[index + 1][1];
                } else {
                    count[index][0] = count[index + 1][0];
                    count[index][1] = count[index + 1][1];
                    count[index][2] = (count[index + 1][0] + count[index + 1][2]) % MOD;
                }
            }
            return count[0][0];
        }
    }
}
