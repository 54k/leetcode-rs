import java.util.*;

public class Day307 {
    static class Solution {
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
}
