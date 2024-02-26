package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Set;
import java.util.Stack;

public class Day470 {
    // https://leetcode.com/problems/same-tree/description
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    static class Solution1 {
        public boolean isSameTree(TreeNode p, TreeNode q) {
            var stack = new Stack<TreeNode>();
            stack.push(p);
            stack.push(q);
            while (stack.size() >= 2) {
                var a = stack.pop();
                var b = stack.pop();

                if ((a == null && b != null) || (a != null && b == null)
                        || (a != null && b != null && a.val != b.val)) {
                    return false;
                }

                if (a == null && b == null) {
                    continue;
                }

                if (a != null) {
                    stack.push(a.left);
                } else {
                    stack.push(null);
                }

                if (b != null) {
                    stack.push(b.left);
                } else {
                    stack.push(null);
                }

                if (a != null) {
                    stack.push(a.right);
                } else {
                    stack.push(null);
                }

                if (b != null) {
                    stack.push(b.right);
                } else {
                    stack.push(null);
                }
            }

            return stack.isEmpty();
        }
    }

    // https://leetcode.com/problems/distance-to-a-cycle-in-undirected-graph/description/
    static class Solution1 {
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

    // https://leetcode.com/problems/paths-in-maze-that-lead-to-same-room/description/
    static class Solution2 {
        public int numberOfPaths(int n, int[][] corridors) {
            var adj = new HashMap<Integer, Set<Integer>>();
            for (var e : corridors) {
                if (e[0] < e[1])
                    adj.computeIfAbsent(e[0], x -> new HashSet<>()).add(e[1]);
                else
                    adj.computeIfAbsent(e[1], x -> new HashSet<>()).add(e[0]);
            }

            var res = 0;
            for (var e : corridors) {
                var s1 = adj.getOrDefault(e[0], new HashSet<>());
                var s2 = adj.getOrDefault(e[1], new HashSet<>());
                for (var x : s1) {
                    if (s2.contains(x)) {
                        res++;
                    }
                }
            }
            return res;
        }
    }
}
