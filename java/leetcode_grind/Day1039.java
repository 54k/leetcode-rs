package leetcode_grind;

public class Day1039 {
    // https://leetcode.com/problems/compare-version-numbers/description/?envType=daily-question&envId=2025-09-23
    static class Solution1 {
        public int compareVersion(String version1, String version2) {
            String[] nums1 = version1.split("\\.");
            String[] nums2 = version2.split("\\.");
            int n1 = nums1.length, n2 = nums2.length;

            int i1, i2;
            for (int i = 0; i < Math.max(n1, n2); ++i) {
                i1 = i < n1 ? Integer.parseInt(nums1[i]) : 0;
                i2 = i < n2 ? Integer.parseInt(nums2[i]) : 0;
                if (i1 != i2) {
                    return i1 > i2 ? 1 : -1;
                }
            }
            return 0;
        }
    }

    // https://leetcode.com/problems/longest-palindromic-substring/description/
    static class Solution2 {
        public String longestPalindrome(String s) {
            int n = s.length();
            var dp = new boolean[n][n];

            int start = 0;
            int end = 0;

            for (int i = n - 1; i >= 0; i--) {
                for (int j = i + 1; j < n; j++) {
                    dp[i][j] = s.charAt(i) == s.charAt(j) && (j - i <= 2 || dp[i + 1][j - 1]);
                    if (dp[i][j] && j - i > end - start) {
                        start = i;
                        end = j;
                    }
                }
            }

            return s.substring(start, end + 1);
        }
    }

    static class Solution3 {
        public String longestPalindrome(String s) {
            int n = s.length();
            boolean[][] dp = new boolean[n][n];
            int[] ans = new int[] { 0, 0 };

            for (int i = 0; i < n; i++) {
                dp[i][i] = true;
            }

            for (int i = 0; i < n - 1; i++) {
                if (s.charAt(i) == s.charAt(i + 1)) {
                    dp[i][i + 1] = true;
                    ans[0] = i;
                    ans[1] = i + 1;
                }
            }

            for (int diff = 2; diff < n; diff++) {
                for (int i = 0; i < n - diff; i++) {
                    int j = i + diff;

                    if (s.charAt(i) == s.charAt(j) && dp[i + 1][j - 1]) {
                        dp[i][j] = true;
                        ans[0] = i;
                        ans[1] = j;
                    }
                }
            }

            return s.substring(ans[0], ans[1] + 1);
        }
    }

    static class Solution4 {
        public String longestPalindrome(String s) {
            int n = s.length();
            int start = 0;
            int end = 0;

            for (int i = 0; i < n; i++) {
                for (int j = 0; j <= 1; j++) {
                    int left = i;
                    int right = i + j;

                    while (left >= 0 && right <= n - 1 && s.charAt(left) == s.charAt(right)) {
                        --left;
                        ++right;
                    }

                    ++left;
                    --right;

                    if (right - left > end - start) {
                        start = left;
                        end = right;
                    }
                }
            }

            return s.substring(start, end + 1);
        }
    }
}
