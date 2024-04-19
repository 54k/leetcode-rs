package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day524 {
    // https://leetcode.com/problems/number-of-islands/description
    static class Solution1 {
        public int numIslands(char[][] grid) {
            var m = grid.length;
            var n = grid[0].length;
            var ans = 0;
            var dfs = new Object() {
                void dfs(int[] v) {
                    for (var d : new int[][] { { 0, 1 }, { 1, 0 }, { -1, 0 }, { 0, -1 } }) {
                        var u = new int[] { v[0] + d[0], v[1] + d[1] };
                        if (0 <= u[0] && u[0] < m && 0 <= u[1] && u[1] < n && grid[u[0]][u[1]] == '1') {
                            grid[u[0]][u[1]] = '0';
                            dfs(u);
                        }
                    }
                }
            };
            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    if (grid[r][c] == '1') {
                        grid[r][c] = '0';
                        dfs.dfs(new int[] { r, c });
                        ans++;
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/number-of-islands-ii/description/
    static class Solution2 {
        public List<Integer> numIslands2(int m, int n, int[][] positions) {
            class UF {
                int cmp;
                int[] repr;
                int[] rank;

                UF(int n) {
                    cmp = 0;
                    repr = new int[n];
                    rank = new int[n];
                    Arrays.fill(repr, -1);
                    Arrays.fill(rank, 1);
                }

                void add(int x) {
                    if (repr[x] > -1) {
                        return;
                    }
                    repr[x] = x;
                    cmp++;
                }

                int find(int x) {
                    if (repr[x] == -1) {
                        return -1;
                    }
                    if (repr[x] != x) {
                        repr[x] = find(repr[x]);
                    }
                    return repr[x];
                }

                void union(int x, int y) {
                    int px = find(x), py = find(y);
                    if (px == py) {
                        return;
                    }
                    if (rank[px] < rank[py]) {
                        repr[px] = py;
                        rank[py] += rank[px];
                    } else {
                        repr[py] = px;
                        rank[px] += rank[py];
                    }
                    cmp--;
                }
            }

            var ans = new ArrayList<Integer>();
            var uf = new UF(n * m);
            for (var pos : positions) {
                int r = pos[0], c = pos[1];
                int v = r * n + c;
                uf.add(v);
                for (var d : new int[][] { { 0, 1 }, { 0, -1 }, { 1, 0 }, { -1, 0 } }) {
                    int ru = r + d[0], cu = c + d[1];
                    int u = ru * n + cu;
                    if (0 <= ru && ru < m && 0 <= cu && cu < n && uf.find(u) != -1) {
                        uf.union(v, u);
                    }
                }
                ans.add(uf.cmp);
            }
            return ans;
        }
    }
}
