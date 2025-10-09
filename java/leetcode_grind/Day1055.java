package leetcode_grind;

public class Day1055 {
    // https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions/description/?envType=daily-question&envId=2025-10-09
    static class Solution1 {
        public long minTime(int[] skill, int[] mana) {
            int n = skill.length;
            int m = mana.length;
            long[] times = new long[n];

            for (int j = 0; j < m; j++) {
                long curTime = 0;
                for (int i = 0; i < n; i++) {
                    curTime = Math.max(curTime, times[i]) + (long) skill[i] * mana[j];
                }
                times[n - 1] = curTime;
                for (int i = n - 2; i >= 0; i--) {
                    times[i] = times[i + 1] - (long) skill[i + 1] * mana[j];
                }
            }
            return times[n - 1];
        }
    }
}
