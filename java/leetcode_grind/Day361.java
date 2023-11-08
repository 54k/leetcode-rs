package leetcode_grind;

import java.util.Arrays;

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

    // https://leetcode.com/problems/longest-palindromic-subsequence-ii/description/
    // https://leetcode.com/problems/longest-palindromic-subsequence-ii/solutions/981739/java-recursive-tle-memoization-3d-bottom-up-2d-bottom-up-o-n-space/
    static class Solution2 {
        public int longestPalindromeSubseqTopDown(String s) {
            var memo = new int[s.length()][s.length()][27];
            for (var r = 0; r < memo.length; r++) {
                for (var c = 0; c < memo[r].length; c++) {
                    Arrays.fill(memo[r][c], -1);
                }
            }

            var dp = new Object() {
                int apply(int i, int j, int prev) {
                    if (i >= j) {
                        return 0;
                    }

                    if (memo[i][j][prev] != -1) {
                        return memo[i][j][prev];
                    }

                    if (s.charAt(i) - 'a' == prev) {
                        return apply(i + 1, j, prev);
                    }
                    if (s.charAt(j) - 'a' == prev) {
                        return apply(i, j - 1, prev);
                    }

                    if (s.charAt(i) == s.charAt(j)) {
                        memo[i][j][prev] = apply(i + 1, j - 1, s.charAt(i) - 'a') + 2;
                    } else {
                        memo[i][j][prev] = Math.max(
                                apply(i + 1, j, prev),
                                apply(i, j - 1, prev));
                    }

                    return memo[i][j][prev];
                }
            };
            return dp.apply(0, s.length() - 1, 26);
        }

        public int longestPalindromeSubseqBottomUp3d(String s) {
            var dp = new int[s.length()][s.length()][27];

            for (int i = s.length() - 1; i >= 0; i--) {
                for (int j = i + 1; j < s.length(); j++) {
                    for (int prev = 0; prev <= 26; prev++) {
                        if (s.charAt(i) - 'a' == prev) {
                            dp[i][j][prev] = dp[i + 1][j][prev];
                            continue;
                        }
                        if (s.charAt(j) - 'a' == prev) {
                            dp[i][j][prev] = dp[i][j - 1][prev];
                            continue;
                        }

                        if (s.charAt(i) == s.charAt(j)) {
                            dp[i][j][prev] = dp[i + 1][j - 1][s.charAt(i) - 'a'] + 2;
                        } else {
                            dp[i][j][prev] = Math.max(
                                    dp[i + 1][j][prev],
                                    dp[i][j - 1][prev]);
                        }
                    }
                }
            }

            return dp[0][s.length() - 1][26];
        }
    }
}
