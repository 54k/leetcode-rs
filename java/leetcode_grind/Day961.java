package leetcode_grind;

import java.util.Arrays;
import java.util.PriorityQueue;

public class Day961 {
    // https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/description/?envType=daily-question&envId=2025-07-07
    static class Solution1 {
        public int maxEvents(int[][] events) {
            int n = events.length;
            int maxDay = 0;
            for (int[] event : events) {
                maxDay = Math.max(maxDay, event[1]);
            }

            PriorityQueue<Integer> pq = new PriorityQueue<>();
            Arrays.sort(events, (a, b) -> a[0] - b[0]);
            int ans = 0;

            for (int i = 1, j = 0; i <= maxDay; i++) {
                while (j < n && events[j][0] <= i) {
                    pq.offer(events[j][1]);
                    j++;
                }

                while (!pq.isEmpty() && pq.peek() < i) {
                    pq.poll();
                }

                if (!pq.isEmpty()) {
                    pq.poll();
                    ans++;
                }
            }

            return ans;
        }
    }
}
