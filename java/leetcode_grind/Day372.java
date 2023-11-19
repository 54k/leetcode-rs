package leetcode_grind;

import java.util.Arrays;

public class Day372 {
    // https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description
    static class Solution1 {
        public int minimumDeleteSum(String s1, String s2) {
            var m = s1.length();
            var n = s2.length();
            var dp = new int[m + 1][n + 1];

            for (int i = 1; i <= m; ++i) {
                dp[i][0] = dp[i - 1][0] + s1.charAt(i - 1);
                for (int j = 1; j <= n; ++j) {
                    if (i == 1) {
                        dp[0][j] = dp[0][j - 1] + s2.charAt(j - 1);
                    }

                    if (s1.charAt(i - 1) == s2.charAt(j - 1)) {
                        dp[i][j] = dp[i - 1][j - 1];
                    } else {
                        dp[i][j] = Math.min(dp[i - 1][j] + s1.charAt(i - 1), dp[i][j - 1] + s2.charAt(j - 1));
                    }
                }
            }

            return dp[m][n];
        }
    }

    // https://leetcode.com/problems/distinct-subsequences/description
    static class Solution2 {
        public int numDistinct1(String s, String t) {
            var m = s.length();
            var n = t.length();
            var memo = new int[m][n];
            for (int i = 0; i < m; ++i) {
                Arrays.fill(memo[i], -1);
            }

            var dp = new Object() {
                int apply(int i, int j) {
                    if (i == m || j == n) {
                        return j == n ? 1 : 0;
                    }
                    if (m - i < n - j) {
                        return 0;
                    }
                    if (memo[i][j] != -1) {
                        return memo[i][j];
                    }

                    var ans = 0;
                    if (s.charAt(i) == t.charAt(j)) {
                        ans += apply(i + 1, j + 1);
                    }
                    ans += apply(i + 1, j);

                    return memo[i][j] = ans;
                }
            };

            return dp.apply(0, 0);
        }

        public int numDistinct2(String s, String t) {
            var m = s.length();
            var n = t.length();
            var dp = new int[m + 1][n + 1];
            dp[0][0] = 1;

            for (int i = 1; i <= m; i++) {
                dp[i][0] = 1;
                for (int j = 1; j <= n; j++) {
                    dp[i][j] += dp[i - 1][j];
                    if (s.charAt(i - 1) == t.charAt(j - 1)) {
                        dp[i][j] += dp[i - 1][j - 1];
                    }
                }
            }

            return dp[m][n];
        }
    }

    // https://leetcode.com/problems/palindromic-substrings/description/
    static class Solution3 {
        public int countSubstrings1(String s) {
            var n = s.length();
            var dp = new boolean[n][n];
            var ans = s.length();
            for (int i = n - 1; i >= 0; i--) {
                dp[i][i] = true;
                for (int j = i + 1; j < n; j++) {
                    if (s.charAt(i) == s.charAt(j) && (j - i <= 2 || dp[i + 1][j - 1])) {
                        dp[i][j] = true;
                    }
                    ans += dp[i][j] ? 1 : 0;
                }
            }

            return ans;
        }

        public int countSubstrings2(String s) {
            var n = s.length();
            var dp = new boolean[n][n];
            var ans = 0;
            for (int i = 0; i < n; ++i) {
                dp[i][i] = true;
                ans++;
            }

            for (int L = 2; L <= n; L++) {
                for (int i = 0; i <= n - L; i++) {
                    int j = i + L - 1;
                    if (s.charAt(i) == s.charAt(j) && (j - i <= 2 || dp[i + 1][j - 1])) {
                        dp[i][j] = true;
                    }
                    ans += dp[i][j] ? 1 : 0;
                }
            }
            return ans;
        }

        public int countSubstrings3(String s) {
            var n = s.length();
            var ans = 0;

            for (int mid = 0; mid < n; ++mid) {
                for (int j = 0; j <= 1; j++) {
                    var l = mid;
                    var r = mid + j;
                    while (0 <= l && r < n && s.charAt(l) == s.charAt(r)) {
                        ans++;
                        l--;
                        r++;
                    }
                }
            }

            return ans;
        }
    }
}
