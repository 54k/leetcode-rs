package leetcode_grind;

import java.util.Arrays;

public class Day1108 {
    // https://leetcode.com/problems/maximum-running-time-of-n-computers/description/?envType=daily-question&envId=2025-12-01
    static class Solution1 {
        public long maxRunTime(int n, int[] batteries) {
            Arrays.sort(batteries);
            long extra = 0;
            for (int i = 0; i < batteries.length - n; i++) {
                extra += batteries[i];
            }

            int[] live = Arrays.copyOfRange(batteries, batteries.length - n, batteries.length);

            for (int i = 0; i < n - 1; i++) {
                if (extra < (long) (i + 1) * (live[i + 1] - live[i])) {
                    return live[i] + extra / (long) (i + 1);
                }

                extra -= (long) (i + 1) * (live[i + 1] - live[i]);
            }

            return live[n - 1] + extra / n;
        }
    }

}
