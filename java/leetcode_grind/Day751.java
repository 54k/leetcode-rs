package leetcode_grind;

public class Day751 {
    // https://leetcode.com/problems/couples-holding-hands/description/
    static class Solution {
        public int minSwapsCouples(int[] row) {
            int N = row.length / 2;
            int[][] couples = new int[N][2];
            for (int i = 0; i < row.length; i++) {
                add(couples[row[i] / 2], i / 2 + 1);
            }

            // adj[x] = {i, j} means that
            // x-th couch connected to couches i, j (all 1 indexed) by couples
            int[][] adj = new int[N + 1][2];
            for (int[] couple : couples) {
                add(adj[couple[0]], couple[1]);
                add(adj[couple[1]], couple[0]);
            }

            // The answer will be N minus the number of cycles in adj.
            int ans = N;
            for (int r = 1; r <= N; r++) {
                if (adj[r][0] == 0 && adj[r][1] == 0) {
                    continue;
                }

                ans--;
                int x = r, y = pop(adj[r]);
                while (y != r) {
                    rem(adj[y], x);

                    x = y;
                    y = pop(adj[y]);
                }
            }

            return ans;
        }

        void add(int[] pair, int x) {
            pair[pair[0] == 0 ? 0 : 1] = x;
        }

        void rem(int[] pair, int x) {
            pair[pair[0] == x ? 0 : 1] = 0;
        }

        int pop(int[] pair) {
            int x = pair[0];
            if (x != 0) {
                pair[0] = 0;
            } else {
                x = pair[1];
                pair[1] = 0;
            }
            return x;
        }
    }

    static class Solution2 {
        int N;
        int[][] pairs;

        public int minSwapsCouples(int[] row) {
            N = row.length / 2;
            pairs = new int[N][2];
            for (int i = 0; i < N; i++) {
                pairs[i][0] = row[2 * i] / 2;
                pairs[i][1] = row[2 * i + 1] / 2;
            }

            return solve(0);
        }

        void swap(int a, int b, int c, int d) {
            int t = pairs[a][b];
            pairs[a][b] = pairs[c][d];
            pairs[c][d] = t;
        }

        int solve(int i) {
            if (i == N)
                return 0;
            int x = pairs[i][0], y = pairs[i][1];
            if (x == y)
                return solve(i + 1);

            int jx = 0, kx = 0, jy = 0, ky = 0;
            for (int j = i + 1; j < N; ++j) {
                for (int k = 0; k <= 1; ++k) {
                    if (pairs[j][k] == x) {
                        jx = j;
                        kx = k;
                    }
                    if (pairs[j][k] == y) {
                        jy = j;
                        ky = k;
                    }
                }
            }

            swap(i, 1, jx, kx);
            int ans1 = 1 + solve(i + 1);
            swap(i, 1, jx, kx);

            swap(i, 0, jy, ky);
            int ans2 = 1 + solve(i + 1);
            swap(i, 0, jy, ky);

            return Math.min(ans1, ans2);
        }
    }

}
