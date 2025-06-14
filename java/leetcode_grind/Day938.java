package leetcode_grind;

public class Day938 {
    // https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/description/?envType=daily-question&envId=2025-06-14
    static class Solution1 {
        public int minMaxDifference(int num) {
            String s = Integer.toString(num);
            String t = s;
            int pos = 0;
            while (pos < s.length() && s.charAt(pos) == '9') {
                pos++;
            }
            if (pos < s.length()) {
                s = s.replace(s.charAt(pos), '9');
            }
            t = t.replace(t.charAt(0), '0');
            return Integer.parseInt(s) - Integer.parseInt(t);
        }
    }

    // https://leetcode.com/problems/guess-number-higher-or-lower-ii/description/
    static class Solution2 {
        int calculate(int low, int high) {
            if (low >= high) {
                return 0;
            }
            int minres = Integer.MAX_VALUE;
            for (int i = low; i <= high; i++) {
                int res = i + Math.max(calculate(i + 1, high), calculate(low, i - 1));
                minres = Math.min(res, minres);
            }
            return minres;
        }

        public int getMoneyAmount(int n) {
            return calculate(1, n);
        }
    }

    static class Solution3 {
        int calculate(int low, int high) {
            if (low >= high) {
                return 0;
            }
            int minres = Integer.MAX_VALUE;
            for (int i = (low + high) / 2; i <= high; i++) {
                int res = i + Math.max(calculate(i + 1, high), calculate(low, i - 1));
                minres = Math.min(res, minres);
            }
            return minres;
        }

        public int getMoneyAmount(int n) {
            return calculate(1, n);
        }
    }

    static class Solution4 {
        public int getMoneyAmount(int n) {
            int[][] dp = new int[n + 1][n + 1];
            for (int len = 2; len <= n; len++) {
                for (int start = 1; start <= n - len + 1; start++) {
                    int minres = Integer.MAX_VALUE;
                    for (int piv = start; piv < start + len - 1; piv++) {
                        int res = piv + Math.max(dp[start][piv - 1], dp[piv + 1][start + len - 1]);
                        minres = Math.min(res, minres);
                    }
                    dp[start][start + len - 1] = minres;
                }
            }
            return dp[1][n];
        }
    }

    static class Solution5 {
        public int getMoneyAmount(int n) {
            int[][] dp = new int[n + 1][n + 1];
            for (int len = 2; len <= n; len++) {
                for (int start = 1; start <= n - len + 1; start++) {
                    int minres = Integer.MAX_VALUE;
                    for (int piv = start + (len - 1) / 2; piv < start + len - 1; piv++) {
                        int res = piv + Math.max(dp[start][piv - 1], dp[piv + 1][start + len - 1]);
                        minres = Math.min(res, minres);
                    }
                    dp[start][start + len - 1] = minres;
                }
            }
            return dp[1][n];
        }
    }
}
