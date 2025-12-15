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
}
