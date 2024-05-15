package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

public class day548 {
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
}
