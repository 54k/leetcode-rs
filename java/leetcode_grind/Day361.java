package leetcode_grind;

public class Day361 {
    // https://leetcode.com/problems/longest-palindromic-subsequence/description/
    static class Solution1 {
        public int longestPalindromeSubseqTopDown(String s) {
            var memo = new Integer[s.length()][s.length()];
            var dp = new Object() {
                int apply(int left, int right) {
                    if (left == right) {
                        return 1;
                    }

                    if (left > right) {
                        return 0;
                    }

                    if (memo[left][right] != null) {
                        return memo[left][right];
                    }

                    if (s.charAt(left) == s.charAt(right)) {
                        memo[left][right] = apply(left + 1, right - 1) + 2;
                    } else {
                        memo[left][right] = Math.max(apply(left + 1, right), apply(left, right - 1));
                    }
                    return memo[left][right];
                }
            };

            return dp.apply(0, s.length() - 1);
        }

        public int longestPalindromeSubseqBottomUp(String s) {
            var n = s.length();
            var dp = new int[n][n];
            for (int i = n - 1; i >= 0; i--) {
                dp[i][i] = 1;
                for (int j = i + 1; j < n; j++) {
                    if (s.charAt(i) == s.charAt(j)) {
                        dp[i][j] = 2 + dp[i + 1][j - 1];
                    } else {
                        dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
                    }
                }
            }
            return dp[0][n - 1];
        }

        public int longestPalindromeSubseqBottomUpOptimized(String s) {
            var n = s.length();
            var dp = new int[n];
            var prevDp = new int[n];
            for (var i = n - 1; i >= 0; i--) {
                dp[i] = 1;
                for (var j = i + 1; j < n; j++) {
                    if (s.charAt(i) == s.charAt(j)) {
                        dp[j] = prevDp[j - 1] + 2;
                    } else {
                        dp[j] = Math.max(prevDp[j], dp[j - 1]);
                    }
                }
                prevDp = dp.clone();
            }
            return dp[n - 1];
        }
    }
}
