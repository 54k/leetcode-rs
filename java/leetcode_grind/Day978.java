package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day978 {
    // https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/description/?envType=daily-question&envId=2025-07-24
    static class Solution1 {
        int res = Integer.MAX_VALUE;

        public int minimumScore(int[] nums, int[][] edges) {
            int n = nums.length;
            List<List<Integer>> e = new ArrayList<>();

            for (int i = 0; i < n; i++) {
                e.add(new ArrayList<>());
            }

            for (int[] v : edges) {
                e.get(v[0]).add(v[1]);
                e.get(v[1]).add(v[0]);
            }

            int sum = 0;
            for (int x : nums) {
                sum ^= x;
            }

            dfs(0, -1, nums, e, sum);
            return res;
        }

        int calc(int part1, int part2, int part3) {
            return (Math.max(part1, Math.max(part2, part3)) - Math.min(part1, Math.min(part2, part3)));
        }

        int dfs(int x, int f, int[] nums, List<List<Integer>> e, int sum) {
            int son = nums[x];
            for (int y : e.get(x)) {
                if (y == f) {
                    continue;
                }
                son ^= dfs(y, x, nums, e, sum);
            }

            for (int y : e.get(x)) {
                if (y == f) {
                    dfs2(y, x, son, x, nums, e, sum);
                }
            }

            return son;
        }

        int dfs2(int x, int f, int oth, int anc, int[] nums, List<List<Integer>> e, int sum) {
            int son = nums[x];
            for (int y : e.get(x)) {
                if (y == f) {
                    continue;
                }
                son ^= dfs2(y, x, oth, anc, nums, e, sum);
            }
            if (f == anc) {
                return son;
            }
            res = Math.min(res, calc(oth, son, sum ^ oth ^ son));
            return son;
        }
    }

    static class Solution2 {
        public int minimumScore(int[] nums, int[][] edges) {
            int n = nums.length;
            List<List<Integer>> adj = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                adj.add(new ArrayList<>());
            }
            for (int[] e : edges) {
                adj.get(e[0]).add(e[1]);
                adj.get(e[1]).add(e[0]);
            }

            int[] sum = new int[n];
            int[] in = new int[n];
            int[] out = new int[n];
            int[] cnt = { 0 };

            dfs(0, -1, nums, adj, sum, in, out, cnt);
            int res = Integer.MAX_VALUE;
            for (int u = 1; u < n; u++) {
                for (int v = u + 1; v < n; v++) {
                    if (in[v] > in[u] && in[v] < out[u]) {
                        res = Math.min(res, calc(sum[0] ^ sum[u], sum[u] ^ sum[v], sum[v]));
                    } else if (in[u] > in[v] && in[u] < out[v]) {
                        res = Math.min(res, calc(sum[0] ^ sum[v], sum[v] ^ sum[u], sum[u]));
                    } else {
                        res = Math.min(res, calc(sum[0] ^ sum[u] ^ sum[v], sum[u], sum[v]));
                    }
                }
            }
            return res;
        }

        int calc(int part1, int part2, int part3) {
            return (Math.max(part1, Math.max(part2, part3)) -
                    Math.min(part1, Math.min(part2, part3)));
        }

        void dfs(
                int x,
                int fa,
                int[] nums,
                List<List<Integer>> adj,
                int[] sum,
                int[] in,
                int[] out,
                int[] cnt) {
            in[x] = cnt[0]++;
            sum[x] = nums[x];
            for (int y : adj.get(x)) {
                if (y == fa) {
                    continue;
                }
                dfs(y, x, nums, adj, sum, in, out, cnt);
                sum[x] ^= sum[y];
            }
            out[x] = cnt[0];
        }
    }

}
