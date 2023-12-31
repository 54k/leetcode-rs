package leetcode_grind;

public class Day414 {
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
    static class Solution1 {
        public int maxProfit(int[] prices) {
            int ans = 0;
            int mn = Integer.MAX_VALUE;
            for (int p : prices) {
                if (p < mn) {
                    mn = p;
                } else {
                    ans = Math.max(ans, p - mn);
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description/
    static class Solution2 {
        public int maxProfit1(int[] prices) {
            int total = 0;
            int valley = Integer.MAX_VALUE;
            int peak = valley;

            for (int i = 0; i < prices.length; i++) {
                if (prices[i] < peak) {
                    total += peak - valley;
                    valley = prices[i];

                    valley = prices[i];
                    peak = valley;
                } else {
                    peak = prices[i];
                }
            }

            total += peak - valley;
            return total;
        }

        public int maxProfit2(int[] prices) {
            int maxprofit = 0;
            for (int i = 1; i < prices.length; i++) {
                if (prices[i] > prices[i - 1]) {
                    maxprofit += prices[i] - prices[i - 1];
                }
            }
            return maxprofit;
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/description
    static class Solution3 {
        public int maxProfit(int[] prices) {
            int sold = Integer.MIN_VALUE, held = Integer.MIN_VALUE, reset = 0;

            for (int price : prices) {
                int preSold = sold;

                sold = held + price;
                held = Math.max(held, reset - price);
                reset = Math.max(reset, preSold);
            }

            return Math.max(sold, reset);
        }
    }
}
