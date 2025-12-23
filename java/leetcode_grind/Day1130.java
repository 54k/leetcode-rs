package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.PriorityQueue;

public class Day1130 {
    // https://leetcode.com/problems/two-best-non-overlapping-events/description/?envType=daily-question&envId=2025-12-23
    static class Solution1 {
        public int maxTwoEvents(int[][] events) {
            Arrays.sort(events, (a, b) -> a[0] - b[0]);
            int[][] dp = new int[events.length][3];
            for (int[] d : dp)
                Arrays.fill(d, -1);
            return findEvents(events, 0, 0, dp);
        }

        int findEvents(int[][] events, int idx, int cnt, int[][] dp) {
            if (cnt == 2 || idx >= events.length)
                return 0;
            if (dp[idx][cnt] == -1) {
                int end = events[idx][1];
                int lo = idx + 1, hi = events.length - 1;
                while (lo < hi) {
                    int mid = lo + ((hi - lo) >> 1);
                    if (events[mid][0] > end)
                        hi = mid;
                    else
                        lo = mid + 1;
                }

                int include = events[idx][2]
                        + (lo < events.length && events[lo][0] > end ? findEvents(events, lo, cnt + 1, dp) : 0);
                int exclude = findEvents(events, idx + 1, cnt, dp);
                dp[idx][cnt] = Math.max(include, exclude);
            }
            return dp[idx][cnt];
        }
    }

    // static class Solution2 {
    // public int maxTwoEvents(int[][] events) {
    // PriorityQueue<Pair<Integer, Integer>> pq = new
    // PriorityQueue<>(Comparator.comparingInt(Pair::getKey));
    // Arrays.sort(events, (a, b) -> a[0] - b[0]);
    // int maxVal = 0, maxSum = 0;

    // for (int[] event : events) {
    // while (!pq.isEmpty() && pq.peek().getKey() < event[0]) {
    // maxVal = Math.max(maxVal, pq.peek().getValue());
    // pq.poll();
    // }

    // maxSum = Math.max(maxSum, maxVal + event[2]);
    // pq.add(new Pair<>(event[1], event[2]));
    // }

    // return maxSum;
    // }
    // }

    static class Solution3 {
        public int maxTwoEvents(int[][] events) {
            List<int[]> times = new ArrayList<>();
            for (int[] event : events) {
                times.add(new int[] { event[0], 1, event[2] });
                times.add(new int[] { event[1] + 1, 0, event[2] });
            }
            times.sort((a, b) -> a[0] == b[0] ? Integer.compare(a[1], b[1]) : Integer.compare(a[0], b[0]));

            int ans = 0, maxValue = 0;
            for (int[] timeValue : times) {
                if (timeValue[1] == 1) {
                    ans = Math.max(ans, timeValue[2] + maxValue);
                } else {
                    maxValue = Math.max(maxValue, timeValue[2]);
                }
            }
            return ans;
        }
    }
}
