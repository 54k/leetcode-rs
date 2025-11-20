package leetcode_grind;

import java.util.Arrays;

public class Day1097 {
    // https://leetcode.com/problems/set-intersection-size-at-least-two/description/?envType=daily-question&envId=2025-11-20
    static class Solution1 {
        public int intersectionSizeTwo(int[][] intervals) {
            Arrays.sort(intervals, (a, b) -> a[0] != b[0] ? a[0] - b[0] : b[1] - a[1]);
            int[] todo = new int[intervals.length];
            Arrays.fill(todo, 2);
            int ans = 0, t = intervals.length;
            while (--t >= 0) {
                int s = intervals[t][0];
                int e = intervals[t][1];
                int m = todo[t];
                for (int p = s; p < s + m; ++p) {
                    for (int i = 0; i <= t; i++) {
                        if (todo[i] > 0 && p <= intervals[i][1]) {
                            todo[i]--;
                        }
                    }
                    ans++;
                }
            }
            return ans;
        }
    }
}
