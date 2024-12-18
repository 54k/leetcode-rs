package leetcode_grind;

import java.util.Stack;

public class Day770 {
    // https://leetcode.com/problems/rabbits-in-forest/description/
    static class Solution1 {
        public int numRabbits(int[] answers) {
            int[] count = new int[1000];
            for (int x : answers)
                count[x]++;

            int ans = 0;
            for (int k = 0; k < 1000; k++) {
                ans += Math.floorMod(-count[k], k + 1) + count[k];
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/description/
    static class Solution2 {
        public int[] finalPrices(int[] prices) {
            int n = prices.length;
            int[] result = prices.clone();
            for (int i = 0; i < n; i++) {
                for (int j = i + 1; j < n; j++) {
                    if (prices[j] <= prices[i]) {
                        result[i] = prices[i] - prices[j];
                        break;
                    }
                }
            }
            return result;
        }
    }

    static class Solution3 {
        public int[] finalPrices(int[] prices) {
            int[] result = prices.clone();
            Stack<Integer> stack = new Stack<>();
            for (int i = 0; i < prices.length; i++) {
                while (!stack.isEmpty() && prices[stack.peek()] >= prices[i]) {
                    result[stack.pop()] -= prices[i];
                }
                stack.add(i);
            }
            return result;
        }
    }
}
