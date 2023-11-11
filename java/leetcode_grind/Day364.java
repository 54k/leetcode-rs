package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;

public class Day364 {
    // https://leetcode.com/problems/critical-connections-in-a-network/description
    static class Solution {
        public List<List<Integer>> criticalConnections(int n, List<List<Integer>> connections) {
            var adj = new HashMap<Integer, List<Integer>>();
            for (var conn : connections) {
                adj.putIfAbsent(conn.get(0), new ArrayList<Integer>());
                adj.putIfAbsent(conn.get(1), new ArrayList<Integer>());
                adj.get(conn.get(0)).add(conn.get(1));
                adj.get(conn.get(1)).add(conn.get(0));
            }

            var visited = new HashSet<Integer>();
            var criticalEdges = new ArrayList<List<Integer>>();

            var height = new int[n];
            var d = new int[n];

            var dfs = new Object() {
                void dfs(int v, int p) {
                    visited.add(v);
                    height[v] = d[v] = p == -1 ? 0 : height[p] + 1;

                    for (var u : adj.get(v)) {
                        if (u == p) {
                            continue;
                        }

                        if (visited.contains(u)) { // back edge
                            d[v] = Math.min(d[v], height[u]);
                        } else {
                            dfs(u, v);
                            d[v] = Math.min(d[v], d[u]);
                            if (height[v] < d[u]) {
                                criticalEdges.add(List.of(v, u));
                            }
                        }
                    }
                }
            };

            dfs.dfs(0, -1);
            return criticalEdges;
        }
    }
}
