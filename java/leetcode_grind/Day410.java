package leetcode_grind;

import java.util.Arrays;

public class Day410 {
    // https://leetcode.com/problems/minimum-time-to-make-rope-colorful/description
    static class Solution1 {
        public int minCost(String colors, int[] neededTime) {
            int totalTime = 0;
            int i = 0, j = 0;

            while (i < neededTime.length && j < neededTime.length) {
                int currTotal = 0, currMax = 0;

                while (j < neededTime.length && colors.charAt(i) == colors.charAt(j)) {
                    currTotal += neededTime[j];
                    currMax = Math.max(currMax, neededTime[j]);
                    j++;
                }

                totalTime += currTotal - currMax;
                i = j;
            }

            return totalTime;
        }
    }

    // https://leetcode.com/problems/house-robber-ii/description/
    static class Solution2 {
        public int rob(int[] nums) {
            if (nums.length == 0) {
                return 0;
            }

            if (nums.length == 1) {
                return nums[0];
            }

            int max1 = robSimple(nums, 0, nums.length - 2);
            int max2 = robSimple(nums, 1, nums.length - 1);

            return Math.max(max1, max2);
        }

        int robSimple(int[] nums, int start, int end) {
            int t1 = 0;
            int t2 = 0;

            for (int i = start; i <= end; i++) {
                int temp = t1;
                int current = nums[i];
                t1 = Math.max(current + t2, t1);
                t2 = temp;
            }

            return t1;
        }
    }

    // https://leetcode.com/problems/longest-common-subsequence/description
    static class Solution3 {
        public int longestCommonSubsequenceBottomUp(String text1, String text2) {
            int[][] dpGrid = new int[text1.length() + 1][text2.length() + 1];

            for (int col = text2.length() - 1; col >= 0; col--) {
                for (int row = text1.length() - 1; row >= 0; row--) {
                    if (text1.charAt(row) == text2.charAt(col)) {
                        dpGrid[row][col] = 1 + dpGrid[row + 1][col + 1];
                    } else {
                        dpGrid[row][col] = Math.max(dpGrid[row + 1][col], dpGrid[row][col + 1]);
                    }
                }
            }

            return dpGrid[0][0];
        }

        public int longestCommonSubsequenceTopDown(String text1, String text2) {
            var memo = new int[text1.length() + 1][text2.length() + 1];
            for (int i = 0; i < text1.length(); i++) {
                Arrays.fill(memo[i], -1);
            }

            var rec = new Object() {
                int apply(int i, int j) {
                    if (i == text1.length()) {
                        return 0;
                    }
                    if (j == text2.length()) {
                        return 0;
                    }

                    if (memo[i][j] != -1) {
                        return memo[i][j];
                    }

                    int ans = 0;
                    if (text1.charAt(i) == text2.charAt(j)) {
                        ans = 1 + apply(i + 1, j + 1);
                    }

                    ans = Math.max(ans, Math.max(apply(i + 1, j), apply(i, j + 1)));
                    return memo[i][j] = ans;
                }
            };

            return rec.apply(0, 0);
        }
    }

    // https://leetcode.com/problems/edit-distance/description
    static class Solution4 {
        public int minDistanceBottomUp(String word1, String word2) {
            var m = word1.length();
            var n = word2.length();
            var dp = new int[m + 1][n + 1];

            if (Math.min(m, n) == 0) {
                return Math.max(m, n);
            }

            for (int i = 1; i <= m; i++) {
                dp[i][0] = i;
                for (int j = 1; j <= n; j++) {
                    if (i == 1) {
                        dp[0][j] = j;
                    }

                    var ch1 = word1.charAt(i - 1);
                    var ch2 = word2.charAt(j - 1);

                    if (ch1 == ch2) {
                        dp[i][j] = dp[i - 1][j - 1];
                    } else {
                        var replace = dp[i - 1][j - 1];
                        var insert = dp[i][j - 1];
                        var delete = dp[i - 1][j];

                        dp[i][j] = 1 + Math.min(insert, Math.min(replace, delete));
                    }
                }
            }

            return dp[m][n];
        }

        public int minDistanceTopDown(String word1, String word2) {
            var m = word1.length();
            var n = word2.length();

            Integer[][] memo = new Integer[m + 1][n + 1];

            var rec = new Object() {
                int apply(int i, int j) {
                    if (i == m || j == n) {
                        return Math.max(m - i, n - j);
                    }

                    if (memo[i][j] != null) {
                        return memo[i][j];
                    }

                    var ch1 = word1.charAt(i);
                    var ch2 = word2.charAt(j);

                    var ans = 0;
                    if (ch1 == ch2) {
                        ans = apply(i + 1, j + 1);
                    } else {
                        var replace = apply(i + 1, j + 1);
                        var delete = apply(i + 1, j);
                        var insert = apply(i, j + 1);

                        ans = 1 + Math.min(replace, Math.min(delete, insert));
                    }

                    return memo[i][j] = ans;
                }
            };

            return rec.apply(0, 0);
        }
    }
}
