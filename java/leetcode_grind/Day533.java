package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;

public class Day532 {
        // https://leetcode.com/problems/sum-of-distances-in-tree/description
        static class Solution1 {
            public int[] sumOfDistancesInTree(int n, int[][] edges) {
                var ans = new int[n];
                var size = new int[n];
                Arrays.fill(size, 1);
                var adj = new HashMap<Integer, List<Integer>>();
                for ( int i = 0; i < n; i++) {
                    adj.put(i, new ArrayList<Integer>());
                }
                for (var edge : edges) {
                    adj.computeIfAbsent(edge[0], ($) -> new ArrayList<Integer>()).add(edge[1]);
                    adj.computeIfAbsent(edge[1], ($) -> new ArrayList<Integer>()).add(edge[0]);
                }
                var dfs1 = new Object() {
                    void apply(int v, int p) {
                        for (var u : adj.get(v)) {
                            if (u != p) {
                                apply(u, v);
                                size[v] += size[u];
                                ans[v] += ans[u] + size[u];
                            }
                        }
                    }
                };
                var dfs2 = new Object() {
                    void apply(int v, int p) {
                        for (var u : adj.get(v)) {
                            if (u != p) {
                                ans[u] = ans[v] - size[u] + n - size[u];
                                apply(u, v);
                            }
                        }
                    }
                };
                dfs1.apply(0, -1);
                dfs2.apply(0, -1);
                return ans;
            }
        }
}
