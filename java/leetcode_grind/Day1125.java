package leetcode_grind;

import java.util.Arrays;

public class Day1125 {
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-using-strategy/description/?envType=daily-question&envId=2025-12-18
    static class Solution1 {
        public long maxProfit(int[] prices, int[] strategy, int k) {
            int n = prices.length;
            long[] profitSum = new long[n + 1];
            long[] priceSum = new long[n + 1];

            for (int i = 0; i < n; i++) {
                profitSum[i + 1] = profitSum[i] + (long) prices[i] * strategy[i];
                priceSum[i + 1] = priceSum[i] + prices[i];
            }

            long res = profitSum[n];

            for (int i = k - 1; i < n; i++) {
                long leftProfit = profitSum[i - k + 1];
                long righProfit = profitSum[n] - profitSum[i + 1];
                long changeProfit = priceSum[i + 1] - priceSum[i - k / 2 + 1];
                res = Math.max(res, leftProfit + changeProfit + righProfit);
            }

            return res;
        }
    }

    // https://leetcode.com/problems/smallest-range-ii/description/
    static class Solution2 {
        public int smallestRangeII(int[] A, int K) {
            int N = A.length;
            Arrays.sort(A);

            int ans = A[N - 1] - A[0];

            for (int i = 0; i < A.length - 1; i++) {
                int a = A[i], b = A[i + 1];
                int high = Math.max(A[N - 1] - K, a + K);
                int low = Math.min(A[0] + K, b - K);
                ans = Math.min(ans, high - low);
            }

            return ans;
        }
    }
}
