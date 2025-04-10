package leetcode_grind;

import java.util.Arrays;

public class Day874 {
    // https://leetcode.com/problems/count-the-number-of-powerful-integers/submissions/1602524906/?envType=daily-question&envId=2025-04-10
    static class Solution1 {
        public long numberOfPowerfulInt(long start, long finish, int limit, String s) {
            String low = Long.toString(start);
            String high = Long.toString(finish);
            int n = high.length();

            low = String.format("%" + n + "s", low).replace(' ', '0');
            int pre_len = n - s.length();
            long[] memo = new long[n];
            Arrays.fill(memo, -1);
            return dfs(0, true, true, low, high, limit, s, pre_len, memo);
        }

        long dfs(int i, boolean limit_low, boolean limit_high, String low, String high, int limit, String s,
                int pre_len, long[] memo) {
            if (i == low.length()) {
                return 1;
            }
            if (!limit_low && !limit_high && memo[i] != -1) {
                return memo[i];
            }

            int lo = limit_low ? low.charAt(i) - '0' : 0;
            int hi = limit_high ? high.charAt(i) - '0' : 9;
            long res = 0;

            if (i < pre_len) {
                for (int digit = lo; digit <= Math.min(hi, limit); digit++) {
                    res += dfs(i + 1, limit_low && digit == lo, limit_high && digit == hi, low, high, limit, s, pre_len,
                            memo);
                }
            } else {
                int x = s.charAt(i - pre_len) - '0';
                if (lo <= x && x <= Math.min(hi, limit)) {
                    res = dfs(i + 1, limit_low && x == lo, limit_high && x == hi, low, high, limit, s, pre_len, memo);
                }
            }

            if (!limit_low && !limit_high) {
                memo[i] = res;
            }
            return res;
        }
    }

    static class Solution2 {
        public long numberOfPowerfulInt(long start, long finish, int limit, String s) {
            String start_ = Long.toString(start - 1);
            String finish_ = Long.toString(finish);
            return calculate(finish_, s, limit) - calculate(start_, s, limit);
        }

        long calculate(String x, String s, int limit) {
            if (x.length() < s.length()) {
                return 0;
            }
            if (x.length() == s.length()) {
                return x.compareTo(s) >= 0 ? 1 : 0;
            }

            String suffix = x.substring(x.length() - s.length());
            long count = 0;
            int preLen = x.length() - s.length();

            for (int i = 0; i < preLen; i++) {
                int digit = x.charAt(i) - '0';
                if (limit < digit) {
                    count += (long) Math.pow(limit + 1, preLen - i);
                    return count;
                }
                count += (long) (digit) * (long) Math.pow(limit + 1, preLen - 1 - i);
            }

            if (suffix.compareTo(s) >= 0) {
                count++;
            }
            return count;
        }
    }
}
