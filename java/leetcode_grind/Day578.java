package leetcode_grind;

import java.util.ArrayList;
import java.util.PriorityQueue;

public class Day578 {
    // https://leetcode.com/problems/ipo/description/
    static class Solution1 {
        public int findMaximizedCapital(int k, int w, int[] profits, int[] capital) {
            var n = profits.length;
            var pq = new PriorityQueue<int[]>((a, b) -> {
                return b[0] - a[0];
            });
            for (int i = 0; i < n; i++) {
                pq.add(new int[] { profits[i], capital[i] });
            }
            while (k > 0 && !pq.isEmpty()) {
                var lst = new ArrayList<int[]>();
                while (!pq.isEmpty() && pq.peek()[1] > w) {
                    lst.add(pq.remove());
                }
                if (pq.isEmpty()) {
                    break;
                }
                var p = pq.remove();
                w += p[0];
                k--;
                for (var e : lst) {
                    pq.add(e);
                }
            }
            return w;
        }
    }
}
