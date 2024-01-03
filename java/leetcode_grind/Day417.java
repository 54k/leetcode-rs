package leetcode_grind;

public class Day417 {
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/description/
    static class Solution1 {
        public int maxProfit(int[] prices) {
            int n = prices.length;
            if (n <= 1)
                return 0;

            int leftMin = prices[0];
            int rightMax = prices[n - 1];

            int[] leftProfits = new int[n];
            int[] rightProfits = new int[n + 1];

            for (int i = 1; i < n; i++) {
                leftProfits[i] = Math.max(prices[i] - leftMin, leftProfits[i - 1]);
                leftMin = Math.min(leftMin, prices[i]);

                int r = n - 1 - i;
                rightProfits[r] = Math.max(rightMax - prices[r], rightProfits[r + 1]);
                rightMax = Math.max(rightMax, prices[r]);
            }

            int maxProfit = 0;
            for (int i = 0; i < n; i++) {
                maxProfit = Math.max(maxProfit, leftProfits[i] + rightProfits[i + 1]);
            }
            return maxProfit;
        }

        public int maxProfitOnePassSimulation(int[] prices) {
            int t1Cost = Integer.MAX_VALUE;
            int t1Profit = 0;
            int t2Cost = Integer.MAX_VALUE;
            int t2Profit = 0;

            for (int price : prices) {
                t1Cost = Math.min(t1Cost, price);
                t1Profit = Math.max(t1Profit, price - t1Cost);

                t2Cost = Math.min(t2Cost, price - t1Profit);
                t2Profit = Math.max(t2Profit, price - t2Cost);
            }

            return t2Profit;
        }
    }
}
