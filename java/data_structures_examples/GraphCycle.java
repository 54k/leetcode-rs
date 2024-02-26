package data_structures_examples;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Set;

public class GraphCycle {

    // https://leetcode.com/problems/distance-to-a-cycle-in-undirected-graph/description/
    class Solution {
        public int[] distanceToCycle(int n, int[][] edges) {
            var adj = new HashMap<Integer, List<Integer>>();
            for (var e : edges) {
                adj.computeIfAbsent(e[0], x -> new ArrayList<>()).add(e[1]);
                adj.computeIfAbsent(e[1], x -> new ArrayList<>()).add(e[0]);
            }
            var cycle = new Object() {
                int cstart, cend;
                int[] par = new int[n];
                boolean[] visited = new boolean[n];

                boolean find(int v, int p) {
                    visited[v] = true;
                    par[v] = p;
                    for (var u : adj.getOrDefault(v, new ArrayList<>())) {
                        if (u == p) {
                            continue;
                        }
                        if (!visited[u]) {
                            if (find(u, v)) {
                                return true;
                            }
                        } else {
                            cstart = u;
                            cend = v;
                            return true;
                        }
                    }
                    return false;
                }

                Set<Integer> apply(int v) {
                    find(v, v);
                    var inCycle = new HashSet<Integer>();
                    while (cend != cstart) {
                        inCycle.add(cend);
                        cend = par[cend];
                    }
                    inCycle.add(cstart);
                    return inCycle;
                }
            };

            var set = cycle.apply(0);

            int[] dist = new int[n];
            boolean[] vis = new boolean[n];

            var q = new LinkedList<Integer>();
            for (var i : set) {
                q.add(i);
                vis[i] = true;
            }

            int lvl = 0;
            while (!q.isEmpty()) {
                var sz = q.size();
                while (sz-- > 0) {
                    var cur = q.poll();
                    dist[cur] = lvl;
                    for (var u : adj.getOrDefault(cur, new ArrayList<>())) {
                        if (!vis[u]) {
                            vis[u] = true;
                            q.add(u);
                        }
                    }
                }
                lvl++;
            }

            return dist;
        }
    }
}
