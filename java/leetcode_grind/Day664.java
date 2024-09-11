package leetcode_grind;

public class Day664 {
    // https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/?envType=daily-question&envId=2024-09-11
    static class Solution1 {
        public int minBitFlips(int start, int goal) {
            int count = 0;
            while (start > 0 || goal > 0) {
                if ((start & 1) != (goal & 1)) {
                    count++;
                }
                start >>= 1;
                goal >>= 1;
            }
            return count;
        }
    }

    static class Solution2 {
        public int minBitFlips(int start, int goal) {
            if (start == 0 && goal == 0)
                return 0;
            int flip = (start & 1) != (goal & 1) ? 1 : 0;
            return flip + minBitFlips(start >> 1, goal >> 1);
        }
    }

    // https://leetcode.com/problems/longest-happy-prefix/description/
    static class Solution3 {
        public String longestPrefix(String s) {
            long lhash = 0, rhash = 0;
            long b = 128, p = 1, mod = (long) 1e9 + 7;
            int maxi = 0;

            for (int i = 0; i < s.length() - 1; i++) {
                char lchar = s.charAt(i);
                char rchar = s.charAt(s.length() - i - 1);

                lhash = (lhash * b % mod + lchar) % mod;
                rhash = (rhash + rchar * p % mod) % mod;
                p = (p * b) % mod;

                if (lhash == rhash) {
                    maxi = i + 1;
                }
            }

            return s.substring(0, maxi);
        }
    }

    // https://leetcode.com/problems/shortest-common-supersequence/description/
    static class Solution4 {
        public String shortestCommonSupersequence(String str1, String str2) {
            var n = str1.length();
            var m = str2.length();
            var dp = new int[n + 1][m + 1];

            var lcs = new Object() {
                int apply(String s1, String s2, int n, int m) {
                    for (int i = 1; i <= n; i++) {
                        for (int j = 1; j <= m; j++) {
                            if (s1.charAt(i - 1) == s2.charAt(j - 1)) {
                                dp[i][j] = dp[i - 1][j - 1] + 1;
                            } else {
                                dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
                            }
                        }
                    }
                    return dp[n][m];
                }
            };

            // find lcs
            var x = lcs.apply(str1, str2, n, m);
            // print scs
            var ans = new StringBuilder();
            while (n > 0 && m > 0) {
                if (str1.charAt(n - 1) == str2.charAt(m - 1)) {
                    ans.append(str1.charAt(n - 1));
                    n--;
                    m--;
                } else if (dp[n - 1][m] > dp[n][m - 1]) {
                    ans.append(str1.charAt(n - 1));
                    n--;
                } else {
                    ans.append(str2.charAt(m - 1));
                    m--;
                }
            }

            while (n > 0) {
                ans.append(str1.charAt(n - 1));
                n--;
            }

            while (m > 0) {
                ans.append(str2.charAt(m - 1));
                m--;
            }

            ans.reverse();
            return ans.toString();
        }
    }
}
