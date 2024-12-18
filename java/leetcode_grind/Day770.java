package leetcode_grind;

import java.util.Arrays;
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

    // https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/description/
    static class Solution4 {
        public int maximizeSquareHoleArea(int n, int m, int[] hBars, int[] vBars) {
            int x = find(hBars), y = find(vBars);
            int res = Math.min(x, y) + 1;

            return res * res;
        }

        private int find(int[] arr) {
            Arrays.sort(arr);
            int res = 1, i = 0, n = arr.length;

            while (i < n) {
                int count = 1;

                while (i + 1 < n && arr[i] + 1 == arr[i + 1]) {
                    i++;
                    count++;
                }
                i++;
                res = Math.max(res, count);
            }

            return res;
        }
    }
}
