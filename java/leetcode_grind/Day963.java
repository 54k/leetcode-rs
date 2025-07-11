package leetcode_grind;

public class Day963 {
    // https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i/description/?envType=daily-question&envId=2025-07-09
    static public class Solution1 {

        public int maxFreeTime(
                int eventTime,
                int k,
                int[] startTime,
                int[] endTime) {
            int n = startTime.length;
            int res = 0;
            int[] sum = new int[n + 1];
            for (int i = 0; i < n; i++) {
                sum[i + 1] = sum[i] + endTime[i] - startTime[i];
            }
            for (int i = k - 1; i < n; i++) {
                int right = i == n - 1 ? eventTime : startTime[i + 1];
                int left = i == k - 1 ? 0 : endTime[i - k];
                res = Math.max(res, right - left - (sum[i + 1] - sum[i - k + 1]));
            }
            return res;
        }
    }

    static class Solution2 {
        public int maxFreeTime(int eventTime, int k, int[] startTime, int[] endTime) {
            int n = startTime.length;
            int res = 0;
            int t = 0;
            for (int i = 0; i < n; i++) {
                t += endTime[i] - startTime[i];
                int left = i <= k - 1 ? 0 : endTime[i - k];
                int right = i == n - 1 ? eventTime : startTime[i + 1];
                res = Math.max(res, right - left - t);
                if (i >= k - 1) {
                    t -= endTime[i - k + 1] - startTime[i - k + 1];
                }
            }
            return res;
        }
    }
}
