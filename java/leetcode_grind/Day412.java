package leetcode_grind;

public class Day412 {
    // https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/description
    static class Solution1 {
        public int minDifficultyTopDown(int[] jobDifficulty, int d) {
            var n = jobDifficulty.length;
            if (n < d) {
                return -1;
            }

            int[][] mem = new int[n][d + 1];
            for (int i = 0; i < n; i++) {
                for (int j = 0; j <= d; j++) {
                    mem[i][j] = -1;
                }
            }

            var rec = new Object() {
                int apply(int i, int daysRemaining) {
                    if (mem[i][daysRemaining] != -1) {
                        return mem[i][daysRemaining];
                    }

                    if (daysRemaining == 1) {
                        int res = 0;
                        for (int j = i; j < jobDifficulty.length; j++) {
                            res = Math.max(res, jobDifficulty[j]);
                        }
                        return res;
                    }

                    int res = Integer.MAX_VALUE;
                    int dailyMaxJobDiff = 0;

                    for (int j = i; j < jobDifficulty.length - daysRemaining + 1; j++) {
                        dailyMaxJobDiff = Math.max(dailyMaxJobDiff, jobDifficulty[j]);
                        res = Math.min(res, dailyMaxJobDiff + apply(j + 1, daysRemaining - 1));
                    }

                    return mem[i][daysRemaining] = res;
                }
            };

            return rec.apply(0, d);
        }

        public int minDifficultyBottomUp(int[] jobDifficulty, int d) {
            int n = jobDifficulty.length;
            int[][] minDiff = new int[d + 1][n + 1];

            for (int daysRemaining = 0; daysRemaining <= d; daysRemaining++) {
                for (int i = 0; i < n; i++) {
                    minDiff[daysRemaining][i] = Integer.MAX_VALUE;
                }
            }

            for (int dr = 1; dr <= d; dr++) {
                for (int i = 0; i < n - dr + 1; i++) {
                    int dailyMaxJobDiff = 0;

                    for (int j = i + 1; j < n - dr + 2; j++) {
                        dailyMaxJobDiff = Math.max(dailyMaxJobDiff, jobDifficulty[j - 1]);

                        if (minDiff[dr - 1][j] != Integer.MAX_VALUE) {
                            minDiff[dr][i] = Math.min(minDiff[dr][i], dailyMaxJobDiff + minDiff[dr - 1][j]);
                        }
                    }
                }
            }

            return minDiff[d][0] < Integer.MAX_VALUE ? minDiff[d][0] : -1;
        }
    }
}
