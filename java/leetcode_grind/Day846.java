package leetcode_grind;

import java.util.Arrays;

public class Day846 {
    // https://leetcode.com/problems/maximum-candies-allocated-to-k-children/description/?envType=daily-question&envId=2025-03-14
    static class Solution1 {
        public int maximumCandies(int[] candies, long k) {
            var check = new Object() {
                boolean apply(int lim) {
                    var left = k;
                    for (var c : candies) {
                        if (c / lim != 0)
                            left -= (c / lim);
                    }
                    return left <= 0;
                }
            };

            int lower = 0;
            int upper = 1_000_000_000;

            while (lower + 1 < upper) {
                int mid = lower + (upper - lower) / 2;
                if (check.apply(mid)) {
                    lower = mid;
                } else {
                    upper = mid;
                }
            }
            return lower;
        }
    }

    // https://leetcode.com/problems/kth-ancestor-of-a-tree-node/description/
    static class TreeAncestor {

        int maxD;
        int kthParent[][];

        public TreeAncestor(int n, int[] parent) {
            while ((1 << maxD) <= n) {
                maxD++;
            }

            kthParent = new int[n][maxD];
            for (int i = 0; i < n; i++) {
                Arrays.fill(kthParent[i], -1);
            }

            for (int lg = 0; lg < maxD; lg++) {
                for (int node = 0; node < n; node++) {
                    if (lg == 0) {
                        kthParent[node][lg] = parent[node];
                    } else if (kthParent[node][lg - 1] != -1) {
                        kthParent[node][lg] = kthParent[kthParent[node][lg - 1]][lg - 1];
                    }
                }
            }
        }

        public int getKthAncestor(int node, int k) {
            for (int lg = 0; lg < maxD; lg++) {
                if (((1 << lg) & k) != 0) {
                    node = kthParent[node][lg];
                    if (node == -1) {
                        break;
                    }
                }
            }
            return node;
        }
    }

}
