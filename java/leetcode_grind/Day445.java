package leetcode_grind;

public class Day445 {
    // https://leetcode.com/problems/paint-house/
    static class Solution1 {
        public int minCost(int[][] costs) {
            var n = costs.length;
            var dp = new int[n + 1][3];
            dp[1] = costs[0];

            for (int i = 2; i <= n; i++) {
                for (int j = 0; j < 3; j++) {
                    dp[i][j] = costs[i - 1][j] + Math.min(dp[i - 1][(j + 1) % 3], dp[i - 1][(j + 2) % 3]);
                }
            }

            var min = Integer.MAX_VALUE;
            for (var v : dp[n]) {
                min = Math.min(min, v);
            }

            return min;
        }
    }

    // https://leetcode.com/problems/binary-tree-maximum-path-sum/description/
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
        int maxSum;

        public int maxPathSum(TreeNode root) {
            maxSum = Integer.MIN_VALUE;
            gainFromSubTree(root);
            return maxSum;
        }

        int gainFromSubTree(TreeNode root) {
            if (root == null) {
                return 0;
            }

            int gainFromLeft = Math.max(gainFromSubTree(root.left), 0);
            int gainFromRight = Math.max(gainFromSubTree(root.right), 0);

            maxSum = Math.max(maxSum, gainFromLeft + gainFromRight + root.val);

            return Math.max(gainFromLeft + root.val, gainFromRight + root.val);
        }
    }

    // https://leetcode.com/problems/daily-temperatures/description/
    static class Solution3 {
        public int[] dailyTemperatures(int[] temperatures) {
            int n = temperatures.length;

            int hottest = 0;
            int[] answer = new int[n];

            for (int currDay = n - 1; currDay >= 0; currDay--) {
                int currentTemp = temperatures[currDay];
                if (currentTemp >= hottest) {
                    hottest = currentTemp;
                    continue;
                }

                int days = 1;
                while (temperatures[currDay + days] <= currentTemp) {
                    days += answer[currDay + days];
                }
                answer[currDay] = days;
            }

            return answer;
        }
    }
}
