package data_structures_examples;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class LongestCycleInGraph {
    // https://leetcode.com/problems/find-the-safest-path-in-a-grid/description
    static class Solution1 {
        public int maximumSafenessFactor(List<List<Integer>> grid) {
            var m = grid.size();
            var n = grid.get(0).size();
            int[][] sf = new int[m][n];
            for (var r : sf) {
                Arrays.fill(r, Integer.MAX_VALUE);
            }

            int[] maxsafe = new int[] { 0 };

            var bfs = new Object() {
                boolean valid(int i, int j) {
                    return 0 <= i && i < m && 0 <= j && j < n;
                }

                void apply(List<int[]> thi) {
                    var q = new LinkedList<int[]>();
                    for (var t : thi) {
                        sf[t[0]][t[1]] = 0;
                        q.add(new int[] { t[0], t[1], 0 });
                    }

                    while (!q.isEmpty()) {
                        var curr = q.remove();
                        maxsafe[0] = Math.max(curr[2], maxsafe[0]);
                        for (int[] d : new int[][] { { -1, 0 }, { 1, 0 }, { 0, 1 }, { 0, -1 } }) {
                            var ni = curr[0] + d[0];
                            var nj = curr[1] + d[1];

                            if (valid(ni, nj) && sf[ni][nj] == Integer.MAX_VALUE) {
                                sf[ni][nj] = curr[2] + 1;
                                q.add(new int[] { ni, nj, curr[2] + 1 });
                            }
                        }
                    }
                }

                boolean check(int fact) {
                    if (sf[0][0] < fact || sf[m - 1][n - 1] < fact) {
                        return false;
                    }
                    var vis = new boolean[m][n];
                    for (var r : vis) {
                        Arrays.fill(r, false);
                    }
                    var q = new LinkedList<int[]>();
                    q.add(new int[] { 0, 0 });
                    vis[0][0] = true;
                    while (!q.isEmpty()) {
                        var curr = q.remove();

                        if (curr[0] == m - 1 && curr[1] == n - 1) {
                            return true;
                        }

                        for (int[] d : new int[][] { { -1, 0 }, { 1, 0 }, { 0, 1 }, { 0, -1 } }) {
                            var ni = curr[0] + d[0];
                            var nj = curr[1] + d[1];
                            if (valid(ni, nj) && !vis[ni][nj] && sf[ni][nj] >= fact) {
                                vis[ni][nj] = true;
                                q.add(new int[] { ni, nj });
                            }
                        }

                    }
                    return false;
                }
            };

            var thi = new ArrayList<int[]>();
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (grid.get(i).get(j) == 1) {
                        thi.add(new int[] { i, j });
                    }
                }
            }

            bfs.apply(thi);

            int l = 0;
            int r = maxsafe[0] + 1;

            while (l + 1 < r) {
                int mid = (l + r) / 2;
                if (bfs.check(mid)) {
                    l = mid;
                } else {
                    r = mid;
                }
            }

            return l;
        }
    }

    // https://leetcode.com/problems/longest-cycle-in-a-graph/description/
    static class Solution2 {
        int ans = -1;

        void dfs(int node, int[] edges, Map<Integer, Integer> dist, boolean[] visit) {
            visit[node] = true;
            int neighbor = edges[node];

            if (neighbor != -1 && !visit[neighbor]) {
                dist.put(neighbor, dist.get(node) + 1);
                dfs(neighbor, edges, dist, visit);
            } else if (neighbor != -1 && dist.containsKey(neighbor)) {
                ans = Math.max(ans, dist.get(node) - dist.get(neighbor) + 1);
            }
        }

        public int longestCycle(int[] edges) {
            int n = edges.length;
            boolean[] visit = new boolean[n];

            for (int i = 0; i < n; i++) {
                if (!visit[i]) {
                    Map<Integer, Integer> dist = new HashMap<>();
                    dist.put(i, 1);
                    dfs(i, edges, dist, visit);
                }
            }
            return ans;
        }
    }

    static class Solution3 {
        public int longestCycle(int[] edges) {
            int n = edges.length;
            boolean[] visit = new boolean[n];
            int[] indegree = new int[n];

            for (int edge : edges) {
                if (edge != -1) {
                    indegree[edge]++;
                }
            }

            Queue<Integer> q = new LinkedList<>();
            for (int i = 0; i < n; i++) {
                if (indegree[i] == 0) {
                    q.offer(i);
                }
            }

            while (!q.isEmpty()) {
                int node = q.poll();
                visit[node] = true;
                int neighbor = edges[node];
                if (neighbor != -1) {
                    indegree[neighbor]--;
                    if (indegree[neighbor] == 0) {
                        q.offer(neighbor);
                    }
                }
            }

            int answer = -1;
            for (int i = 0; i < n; i++) {
                if (!visit[i]) {
                    int neighbor = edges[i];
                    int count = 1;
                    visit[i] = true;

                    while (neighbor != i) {
                        visit[neighbor] = true;
                        count++;
                        neighbor = edges[neighbor];
                    }
                    answer = Math.max(answer, count);
                }
            }
            return answer;
        }
    }
}
