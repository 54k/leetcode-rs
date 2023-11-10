package leetcode_grind;

public class Day363 {
    // https://leetcode.com/problems/maximal-square/description
    static class Solution1 {
        public int maximalSquare1(char[][] matrix) {
            var rows = matrix.length;
            var cols = rows > 0 ? matrix[0].length : 0;
            var maxsqlen = 0;
            var dp = new int[matrix.length + 1][matrix[0].length + 1];
            for (int i = 1; i <= rows; i++) {
                for (int j = 1; j <= cols; j++) {
                    if (matrix[i - 1][j - 1] == '1') {
                        dp[i][j] = 1 + Math.min(dp[i][j - 1], Math.min(dp[i - 1][j], dp[i - 1][j - 1]));
                    }
                    maxsqlen = Math.max(maxsqlen, dp[i][j]);
                }
            }

            return maxsqlen * maxsqlen;
        }

        public int maximalSquare2(char[][] matrix) {
            var rows = matrix.length;
            var cols = rows > 0 ? matrix[0].length : 0;
            var dp = new int[cols + 1];
            var maxsqlen = 0;
            var prev = 0;

            for (int i = 1; i <= rows; i++) {
                for (int j = 1; j <= cols; j++) {
                    var temp = dp[j];
                    if (matrix[i - 1][j - 1] == '1') {
                        dp[j] = Math.min(Math.min(dp[j - 1], prev), dp[j]) + 1;
                        maxsqlen = Math.max(maxsqlen, dp[j]);
                    } else {
                        dp[j] = 0;
                    }
                    prev = temp;
                }
            }
            return maxsqlen * maxsqlen;
        }
    }

    // https://leetcode.com/problems/longest-palindromic-substring/description
    static class Solution {
        public String longestPalindrome1(String s) {
            var left = 0;
            var right = 0;

            var dp = new boolean[s.length()][s.length()];
            for (var i = s.length() - 1; i >= 0; i--) {
                for (var j = i+1; j < s.length(); j++) {
                    if (s.charAt(i) == s.charAt(j) && (j - i <= 2 || dp[i + 1][j - 1])) {
                        dp[i][j] = true;
                        if (j - i > right - left) {
                            left = i;
                            right = j;
                        }
                    }
                }
            }

            return s.substring(left, right + 1);
        }

        public String longestPalindrome2(String s) {
            var left = 0;
            var right = 0;
            for (var mid = 0; mid < s.length() - 1; mid++) {
                for (var j = 0; j < 2; j++) {
                    var l = mid;
                    var r = mid + j;
                    while (0 <= l && s.length() - 1 >= r && s.charAt(l) == s.charAt(r)) {
                        l--;
                        r++;
                    }
                    l++;
                    r--;
                    if (r - l > right - left) {
                        left = l;
                        right = r;
                    }
                }
            }

            return s.substring(left, right + 1);
        }
    }
}
