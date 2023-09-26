package grind;
import java.util.*;

public class Day307 {
    static class Solution1 {
        public int minCostConnectPoints(int[][] points) {
            class DSU {
                int[] repr;
                int[] size;

                DSU(int n) {
                    repr = new int[n];
                    for (int i = 0; i < n; i++) {
                        repr[i] = i;
                    }
                    size = new int[n];
                }

                int find(int x) {
                    if (repr[x] != x) {
                        repr[x] = find(repr[x]);
                    }
                    return repr[x];
                }

                void union(int x, int y) {
                    x = find(x);
                    y = find(y);
                    if (x == y) {
                        return;
                    }

                    if (size[x] > size[y]) {
                        var t = x;
                        x = y;
                        y = t;
                    }

                    repr[x] = y;
                    size[y] += size[x];

                }
            }

            var edges = new ArrayList<int[]>();
            for (var i = 0; i < points.length; i++) {
                for (var j = i + 1; j < points.length; j++) {
                    var dist = Math.abs(points[i][0] - points[j][0]) + Math.abs(points[i][1] - points[j][1]);
                    edges.add(new int[] { dist, i, j });
                }
            }
            Collections.sort(edges, new Comparator<int[]>() {
                @Override
                public int compare(int[] o1, int[] o2) {
                    return o1[0] - o2[0];
                }
            });
            var total = 0;
            var uf = new DSU(points.length);
            for (var e : edges) {
                if (uf.find(e[1]) != uf.find(e[2])) {
                    uf.union(e[1], e[2]);
                    total += e[0];
                }
            }
            return total;
        }
    }

    static class Solution2 {
        public int minCostConnectPoints(int[][] points) {
            var pq = new PriorityQueue<int[]>(new Comparator<int[]>() {
                @Override
                public int compare(int[] o1, int[] o2) {
                    return o1[0] - o2[0];
                }
            });
            var total = 0;
            var n = points.length;
            var used = 0;
            var inMST = new boolean[n];
            pq.add(new int[] { 0, 0 });
            while (used < n) {
                var top = pq.remove();
                var weight = top[0];
                var current = top[1];
                if (inMST[current]) {
                    continue;
                }

                used++;
                inMST[current] = true;
                total += weight;
                for (var next = 0; next < n; next++) {
                    if (!inMST[next]) {
                        var dist = Math.abs(points[next][0] - points[current][0])
                                + Math.abs(points[next][1] - points[current][1]);
                        pq.add(new int[] { dist, next });
                    }
                }
            }
            return total;
        }
    }

    static class Solution3 {
        public int minCostConnectPoints(int[][] points) {
            var n = points.length;
            var inMST = new boolean[n];
            var used = 0;
            var minDist = new int[n];
            for (int i = 1; i < n; i++) {
                minDist[i] = Integer.MAX_VALUE;
            }
            var totalCost = 0;
            while (used < n) {
                var minCost = Integer.MAX_VALUE;
                var minIndex = -1;
                for (var i = 0; i < n; i++) {
                    if (!inMST[i] && minDist[i] < minCost) {
                        minCost = minDist[i];
                        minIndex = i;
                    }
                }

                inMST[minIndex] = true;
                totalCost += minCost;
                used++;

                for (var next = 0; next < n; next++) {
                    var dist = Math.abs(points[minIndex][0] - points[minIndex][0])
                            + Math.abs(points[minIndex][1] - points[next][1]);
                    if (!inMST[next] && minDist[next] > dist) {
                        minDist[next] = dist;
                    }
                }

            }

            return totalCost;
        }
    }
}
