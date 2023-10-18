package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class Day340 {
    // https://leetcode.com/problems/parallel-courses-iii/description/
    static class Solution {
        public int minimumTime(int n, int[][] relations, int[] time) {
            var inDegree = new int[n];
            var adj = new HashMap<Integer, List<Integer>>();
            for (var i = 0; i < n; i++) {
                adj.put(i, new ArrayList<>());
            }

            for (var relation : relations) {
                var from = relation[0] - 1;
                var to = relation[1] - 1;
                inDegree[to]++;
                adj.get(from).add(to);
            }

            var ans = 0;
            var dist = new int[n];
            var queue = new ArrayDeque<Integer>();
            for (var i = 0; i < inDegree.length; i++) {
                if (inDegree[i] == 0) {
                    queue.addLast(i);
                    dist[i] = time[i];
                    ans = Math.max(dist[i], ans);
                }
            }
            while (queue.size() > 0) {
                var curr = queue.pollFirst();
                for (var next : adj.get(curr)) {
                    dist[next] = Math.max(dist[next], time[next] + dist[curr]);
                    ans = Math.max(dist[next], ans);
                    if (--inDegree[next] == 0) {
                        queue.addLast(next);
                    }
                }
            }
            return ans;
        }
    }

}
