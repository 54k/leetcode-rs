package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day1075 {
    // https://leetcode.com/problems/minimum-subarrays-in-a-valid-split/description/?envType=weekly-question&envId=2025-10-29
    static class Solution1 {
        int INF = 1000000000;

        public int validSubarraySplit(int[] nums) {
            int n = nums.length;
            int[] dp = new int[n + 1];
            Arrays.fill(dp, INF);
            dp[0] = 0;

            for (int i = 1; i <= n; i++) {
                for (int j = 1; j <= i; j++) {
                    if (gcd(nums[i - 1], nums[j - 1]) > 1) {
                        dp[i] = Math.min(dp[i], dp[j - 1] + 1);
                    }
                }
            }

            return dp[n] == INF ? -1 : dp[n];
        }

        int gcd(int a, int b) {
            return b == 0 ? a : gcd(b, a % b);
        }
    }

    // https://leetcode.com/problems/smallest-number-with-all-set-bits/description/?envType=daily-question&envId=2025-10-29
    static class Solution2 {
        public int smallestNumber(int n) {
            int x = 1;
            while (x < n) {
                x = x * 2 + 1;
            }
            return x;
        }
    }

    // https://leetcode.com/problems/roman-to-integer/description/
    static class Solution3 {
        static Map<String, Integer> values = new HashMap<>();
        static {
            values.put("M", 1000);
            values.put("D", 500);
            values.put("C", 100);
            values.put("L", 50);
            values.put("X", 10);
            values.put("V", 5);
            values.put("I", 1);
        }

        public int romanToInt(String s) {
            int sum = 0;
            int i = 0;
            while (i < s.length()) {
                String currentSymbol = s.substring(i, i + 1);
                int currentValue = values.get(currentSymbol);
                int nextValue = 0;
                if (i + 1 < s.length()) {
                    String nextSymbol = s.substring(i + 1, i + 2);
                    nextValue = values.get(nextSymbol);
                }

                if (currentValue < nextValue) {
                    sum += nextValue - currentValue;
                    i += 2;
                } else {
                    sum += currentValue;
                    i += 1;
                }
            }
            return sum;
        }
    }

    static class Solution4 {
        static Map<String, Integer> values = new HashMap<>();
        static {
            values.put("M", 1000);
            values.put("D", 500);
            values.put("C", 100);
            values.put("L", 50);
            values.put("X", 10);
            values.put("V", 5);
            values.put("I", 1);
            values.put("IV", 4);
            values.put("IX", 9);
            values.put("XL", 40);
            values.put("XC", 90);
            values.put("CD", 400);
            values.put("CM", 900);
        }

        public int romanToInt(String s) {
            int sum = 0;
            int i = 0;
            while (i < s.length()) {
                if (i < s.length() - 1) {
                    String doubleSymbol = s.substring(i, i + 2);
                    if (values.containsKey(doubleSymbol)) {
                        sum += values.get(doubleSymbol);
                        i += 2;
                        continue;
                    }
                }
                String singleSymbol = s.substring(i, i + 1);
                sum += values.get(singleSymbol);
                i += 1;
            }
            return sum;
        }
    }

    static class Solution5 {
        static Map<String, Integer> values = new HashMap<>();
        static {
            values.put("M", 1000);
            values.put("D", 500);
            values.put("C", 100);
            values.put("L", 50);
            values.put("X", 10);
            values.put("V", 5);
            values.put("I", 1);
        }

        public int romanToInt(String s) {
            String lastSymbol = s.substring(s.length() - 1);
            int lastValue = values.get(lastSymbol);
            int total = lastValue;

            for (int i = s.length() - 2; i >= 0; i--) {
                String currentSymbol = s.substring(i, i + 1);
                int currentValue = values.get(currentSymbol);
                if (currentValue < lastValue) {
                    total -= currentValue;
                } else {
                    total += currentValue;
                }
                lastValue = currentValue;
            }
            return total;
        }
    }
}
