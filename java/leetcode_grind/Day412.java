package leetcode_grind;

public class Day412 {
    // https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/description
    static class Solution1 {
        public int minDifficulty(int[] jobDifficulty, int d) {
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
    }

}
