package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1045 {
    // https://leetcode.com/problems/divide-array-into-increasing-sequences/description/?envType=weekly-question&envId=2025-09-29
    static class Solution1 {
        public boolean canDivideIntoSubsequences(int[] nums, int k) {
            int pre = nums[0];
            int cnt = 0;

            for (int n : nums) {
                if (n == pre) {
                    cnt++;
                } else {
                    pre = n;
                    cnt = 1;
                }

                if (cnt * k > nums.length) {
                    return false;
                }
            }

            return true;
        }
    }

    // https://leetcode.com/problems/minimum-score-triangulation-of-polygon/description/?envType=daily-question&envId=2025-09-29
    static class Solution2 {
        int n;
        int[] values;
        Map<Integer, Integer> memo = new HashMap<Integer, Integer>();

        public int minScoreTriangulation(int[] values) {
            this.n = values.length;
            this.values = values;
            return dp(0, n - 1);
        }

        int dp(int i, int j) {
            if (i + 2 > j) {
                return 0;
            }

            if (i + 2 == j) {
                return values[i] * values[i + 1] * values[j];
            }

            int key = i * n + j;
            if (!memo.containsKey(key)) {
                int minScore = Integer.MAX_VALUE;
                for (int k = i + 1; k < j; k++) {
                    minScore = Math.min(minScore, values[i] * values[k] * values[j] + dp(i, k) + dp(k, j));
                }
                memo.put(key, minScore);
            }
            return memo.get(key);
        }
    }

    static class Solution3 {
        public int minScoreTriangulation(int[] values) {
            int n = values.length;
            int[][] dp = new int[n][n];

            for (int l = n - 1; l >= 0; l--) {
                for (int r = l + 2; r < n; r++) {
                    int score = Integer.MAX_VALUE;
                    for (int k = l + 1; k < r; k++) {
                        score = Math.min(score,
                                values[l] * values[k] * values[r] + dp[l][k] + dp[k][r]);
                    }
                    dp[l][r] = score;
                }
            }

            return dp[0][n - 1];
        }
    }

}
