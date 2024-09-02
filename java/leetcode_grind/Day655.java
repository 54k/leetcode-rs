package leetcode_grind;

public class Day655 {
    // https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/description/?envType=daily-question&envId=2024-09-02
    static class Solution1 {
        public int chalkReplacer(int[] chalk, int k) {
            var n = chalk.length;
            var pref = new long[n + 1];
            for (int i = 1; i <= n; i++) {
                pref[i] = (long) chalk[i - 1] + pref[i - 1];
            }
            long K = (long) k % pref[n];
            int lo = -1, hi = n + 2;
            while (lo + 1 < hi) {
                int mid = (lo + hi) / 2;
                if (pref[mid] <= K) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }
    }
}
