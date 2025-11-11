package leetcode_grind;

public class Day1088 {
    // https://leetcode.com/problems/ones-and-zeroes/description/?envType=daily-question&envId=2025-11-11
    static class Solution1 {
        public int findMaxForm(String[] strs, int m, int n) {
            var cnt = new Object() {
                int[] apply(int i) {
                    int[] ans = new int[2];
                    for (var ch : strs[i].toCharArray()) {
                        ans[ch - '0']++;
                    }
                    return ans;
                }
            };

            int len = strs.length;
            int[][] arr = new int[len][2];
            for (int i = 0; i < len; i++) {
                arr[i] = cnt.apply(i);
            }

            int[][] dp = new int[m + 1][n + 1];
            for (int i = 0; i < len; i++) {
                for (int j = m; j >= arr[i][0]; j--) {
                    for (int k = n; k >= arr[i][1]; k--) {
                        dp[j][k] = Math.max(dp[j][k], dp[j - arr[i][0]][k - arr[i][1]] + 1);
                    }
                }
            }
            return dp[m][n];
        }
    }
}