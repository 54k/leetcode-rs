package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day619 {
    // https://leetcode.com/problems/second-minimum-time-to-reach-destination/description/?envType=daily-question&envId=2024-07-28
    static class Solution1 {
        public int secondMinimum(int n, int[][] edges, int time, int change) {
            Map<Integer, List<Integer>> adj = new HashMap<>();
            for (int[] edge : edges) {
                int a = edge[0], b = edge[1];
                adj.computeIfAbsent(a, value -> new ArrayList<Integer>()).add(b);
                adj.computeIfAbsent(b, value -> new ArrayList<Integer>()).add(a);
            }
            int[] dist1 = new int[n + 1], dist2 = new int[n + 1], freq = new int[n + 1];
            for (int i = 1; i <= n; i++) {
                dist1[i] = dist2[i] = Integer.MAX_VALUE;
                freq[i] = 0;
            }

            PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[1] - b[1]);
            pq.offer(new int[] { 1, 0 });
            dist1[1] = 0;

            while (!pq.isEmpty()) {
                int[] temp = pq.poll();
                int node = temp[0];
                int time_taken = temp[1];

                freq[node]++;

                if (freq[node] == 2 && node == n) {
                    return time_taken;
                }

                if ((time_taken / change) % 2 == 1) {
                    time_taken = change * (time_taken / change + 1) + time;
                } else {
                    time_taken = time_taken + time;
                }

                if (!adj.containsKey(node)) {
                    continue;
                }

                for (int neighbor : adj.get(node)) {
                    if (freq[neighbor] == 2)
                        continue;

                    if (dist1[neighbor] > time_taken) {
                        dist2[neighbor] = dist1[neighbor];
                        dist1[neighbor] = time_taken;
                        pq.offer(new int[] { neighbor, time_taken });
                    } else if (dist2[neighbor] > time_taken && dist1[neighbor] != time_taken) {
                        dist2[neighbor] = time_taken;
                        pq.offer(new int[] { neighbor, time_taken });
                    }
                }
            }
            return 0;
        }
    }
}
