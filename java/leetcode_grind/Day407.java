package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.function.Function;

public class Day407 {

    // https://leetcode.com/problems/string-compression/description/
    static class Solution1 {
        public int compress(char[] chars) {
            int chptr = 0;
            for (int i = 1, group = 1; i <= chars.length; i++) {
                if (i == chars.length || chars[i] != chars[i - 1]) {
                    chars[chptr++] = chars[i - 1];
                    if (group > 1) {
                        for (char c : Integer.toString(group).toCharArray()) {
                            chars[chptr++] = c;
                        }
                    }
                    group = 1;
                } else {
                    group++;
                }
            }
            // System.out.println(Arrays.toString(chars));
            return chptr;
        }
    }

    // https://leetcode.com/problems/minimum-edge-weight-equilibrium-queries-in-a-tree/description/
    static class Solution2 {
        int maxd = 0;

        public int[] minOperationsQueries(int n, int[][] edges, int[][] queries) {
            var adj = new HashMap<Integer, List<int[]>>();
            adj.putIfAbsent(0, new ArrayList<>());
            for (var edge : edges) {
                adj.putIfAbsent(edge[0], new ArrayList<>());
                adj.putIfAbsent(edge[1], new ArrayList<>());
                adj.get(edge[0]).add(new int[] { edge[1], edge[2] });
                adj.get(edge[1]).add(new int[] { edge[0], edge[2] });
            }

            var parent = new int[n];
            var depth = new int[n];
            var freq = new int[n][27];

            var dfs = new Object() {
                void apply(int v, int p, int d) {
                    parent[v] = p;
                    depth[v] = d;
                    for (var u : adj.get(v)) {
                        int nei = u[0];
                        int val = u[1];
                        if (nei != p) {
                            freq[nei] = freq[v].clone();
                            freq[nei][val]++;
                            apply(nei, v, d + 1);
                        }
                    }
                }
            };

            dfs.apply(0, 0, 0);

            while ((1 << maxd) <= n) {
                maxd++;
            }

            var kthparent = new int[n][maxd + 1];

            for (int i = 0; i < maxd; i++) {
                for (int j = 0; j < n; j++) {
                    if (i == 0) {
                        kthparent[j][i] = parent[j];
                    } else if (kthparent[j][i - 1] != -1) {
                        kthparent[j][i] = kthparent[kthparent[j][i - 1]][i - 1];
                    }
                }
            }

            var lca = new Object() {
                int kth(int v, int k) {
                    for (int i = 0; i < maxd; i++) {
                        if (((1 << i) & k) != 0) {
                            v = kthparent[v][i];
                        }
                    }
                    return v;
                }

                int get(int v, int u) {
                    if (depth[v] < depth[u]) {
                        var t = v;
                        v = u;
                        u = t;
                    }

                    int diff = depth[v] - depth[u];
                    v = kth(v, diff);

                    if (v == u) {
                        return v;
                    }

                    for (int i = maxd; i >= 0; i--) {
                        int newV = kthparent[v][i];
                        int newU = kthparent[u][i];

                        if (newV == newU) {
                            continue;
                        }

                        v = newV;
                        u = newU;
                    }

                    return parent[v];
                }
            };

            var m = queries.length;
            int[] answer = new int[m];
            for (int i = 0; i < m; i++) {
                int[] query = queries[i];

                int l = lca.get(query[0], query[1]);
                int length = depth[query[0]] + depth[query[1]] - 2 * depth[l];

                int maxe = 0;
                for (int j = 0; j < 27; j++) {
                    int curr = freq[query[0]][j] + freq[query[1]][j] - 2 * freq[l][j];
                    maxe = Math.max(maxe, curr);
                }
                answer[i] = length - maxe;
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/cycle-length-queries-in-a-tree/description/
    static class Solution3 {
        public int[] cycleLengthQueries(int n, int[][] queries) {
            Function<Integer, Integer> lg = (x) -> {
                var log = (int) (Math.log(x) / Math.log(2));
                // System.out.printf("log %s->%s\n", x, log);
                return log;
            };

            var lca = new Object() {
                int apply(int v, int u) {
                    if (lg.apply(v) > lg.apply(u)) {
                        return apply(u, v);
                    }

                    while (lg.apply(v) < lg.apply(u)) {
                        u /= 2;
                    }

                    while (v != u) {
                        v /= 2;
                        u /= 2;
                    }

                    return v;
                }
            };

            int[] ans = new int[queries.length];
            int i = 0;
            for (var query : queries) {
                int v = query[0];
                int u = query[1];
                int l = lca.apply(v, u);
                // System.out.printf("lca %s->%s %s\n", u, v, l);
                ans[i++] = lg.apply(v) + lg.apply(u) - (l > 1 ? 2 * lg.apply(l) : 0) + 1;
            }
            return ans;
        }
    }s
}
