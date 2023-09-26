package grind;
import java.util.Comparator;
import java.util.PriorityQueue;

public class Day310 {
    // https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/description/
    static class Solution {
        public int[] kWeakestRows(int[][] mat, int k) {
            var rows = new int[mat.length][2];
            for (var i = 0; i < mat.length; i++) {
                var row = mat[i];
                var left = 0;
                var right = mat[0].length;
                while (left < right) {
                    var mid = (left + right) / 2;
                    if (row[mid] == 1) {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                rows[i] = new int[] { left, i };
            }

            var pq = new PriorityQueue<int[]>(new Comparator<int[]>() {
                @Override
                public int compare(int[] o1, int[] o2) {
                    if (o2[0] != o1[0]) {
                        return o2[0] - o1[0];
                    }
                    return o2[1] - o1[1];
                }
            });
            for (var row : rows) {
                pq.add(row);
                if (pq.size() > k) {
                    pq.poll();
                }
            }

            var ans = new int[k];
            while (pq.size() > 0) {
                var e = pq.poll()[1];
                ans[--k] = e;
            }
            return ans;
        }
    }
}
