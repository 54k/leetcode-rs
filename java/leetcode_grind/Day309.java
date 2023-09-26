package leetcode_grind;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.PriorityQueue;

public class Day309 {
    class Solution {
        public int findCheapestPriceBFS(int n, int[][] flights, int src, int dst, int k) {
            var adj = new HashMap<Integer, List<int[]>>();
            for (var flight : flights) {
                var from = flight[0];
                var to = flight[1];
                var dist = flight[2];
                adj.putIfAbsent(from, new ArrayList<>());
                adj.get(from).add(new int[] { to, dist });
            }

            var dist = new int[n];
            for (int i = 0; i < dist.length; i++) {
                dist[i] = Integer.MAX_VALUE;
            }
            dist[src] = 0;

            var queue = new ArrayList<int[]>();
            queue.add(new int[] { src, 0 });

            var lvl = 0;
            while (queue.size() > 0 && lvl <= k) {
                var nextQueue = new ArrayList<int[]>();
                for (var elem : queue) {
                    var from = elem[0];
                    var fromDist = elem[1];
                    if (!adj.containsKey(from)) {
                        continue;
                    }
                    for (var next : adj.get(from)) {
                        var to = next[0];
                        var toDist = next[1];

                        if (dist[to] > fromDist + toDist) {
                            dist[to] = fromDist + toDist;
                            nextQueue.add(new int[] { to, dist[to] });
                        }
                    }
                }
                lvl++;
                queue = nextQueue;
            }

            if (dist[dst] == Integer.MAX_VALUE) {
                return -1;
            }
            return dist[dst];
        }

        public int findCheapestPriceBellmanFord(int n, int[][] flights, int src, int dst, int k) {
            var dist = new int[n];
            for (int i = 0; i < dist.length; i++) {
                dist[i] = Integer.MAX_VALUE;
            }

            dist[src] = 0;
            for (int i = 0; i <= k; i++) {
                var temp = Arrays.copyOf(dist, n);

                for (var flight : flights) {
                    var from = flight[0];
                    var to = flight[1];
                    var cost = flight[2];

                    if (dist[from] != Integer.MAX_VALUE) {
                        temp[to] = Math.min(temp[to], dist[from] + cost);
                    }
                }

                dist = temp;
            }

            if (dist[dst] == Integer.MAX_VALUE) {
                return -1;
            }
            return dist[dst];
        }

        public int findCheapestPriceDijkstra(int n, int[][] flights, int src, int dst, int k) {
            var adj = new HashMap<Integer, List<int[]>>();
            for (var flight : flights) {
                var from = flight[0];
                var to = flight[1];
                var dist = flight[2];
                adj.putIfAbsent(from, new ArrayList<>());
                adj.get(from).add(new int[] { to, dist });
            }

            var stops = new int[n];
            for (int i = 0; i < stops.length; i++) {
                stops[i] = Integer.MAX_VALUE;
            }

            var heap = new PriorityQueue<int[]>(new Comparator<int[]>() {
                @Override
                public int compare(int[] o1, int[] o2) {
                    return o1[0] - o2[0];
                }
            });

            heap.add(new int[] { 0, src, 0 });

            while (!heap.isEmpty()) {
                var v = heap.poll();
                var cost = v[0];
                var from = v[1];
                var steps = v[2];

                if (steps > k + 1 || stops[from] < steps) {
                    continue;
                }

                stops[from] = steps;

                if (from == dst) {
                    return cost;
                }

                if (!adj.containsKey(from)) {
                    continue;
                }

                for (var next : adj.get(from)) {
                    heap.add(new int[] { cost + next[1], next[0], steps + 1 });
                }
            }

            return -1;
        }
    }
}
