package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day467 {
    // https://leetcode.com/problems/cheapest-flights-within-k-stops/description/
    static class Solution1 {
        public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
            Map<Integer, List<int[]>> adj = new HashMap<>();
            for (int[] i : flights) {
                adj.computeIfAbsent(i[0], value -> new ArrayList<>()).add(new int[] { i[1], i[2] });
            }

            int[] stops = new int[n];
            Arrays.fill(stops, Integer.MAX_VALUE);

            PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
            pq.offer(new int[] { 0, src, 0 });

            while (!pq.isEmpty()) {
                int[] temp = pq.poll();
                int dist = temp[0];
                int node = temp[1];
                int steps = temp[2];

                if (steps > stops[node] || steps > k + 1) {
                    continue;
                }

                stops[node] = steps;
                if (node == dst) {
                    return dist;
                }
                if (!adj.containsKey(node)) {
                    continue;
                }
                for (int[] a : adj.get(node)) {
                    pq.offer(new int[] { dist + a[1], a[0], steps + 1 });
                }
            }
            return -1;
        }
    }
}
