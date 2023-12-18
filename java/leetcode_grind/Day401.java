package leetcode_grind;

import java.util.Arrays;

public class Day401 {
    // https://leetcode.com/problems/number-of-longest-increasing-subsequence/description
    static class Solution {
        public int findNumberOfLIS(int[] nums) {
            int[][] dp = new int[nums.length][2];
            int longest = 1;

            for (int i = 0; i < nums.length; i++) {
                dp[i] = new int[] { 1, 1 };
                for (int j = 0; j < i; j++) {
                    if (nums[j] < nums[i]) {
                        if (dp[i][0] < dp[j][0] + 1) {
                            dp[i] = new int[] { dp[j][0] + 1, dp[j][1] };
                            longest = Math.max(longest, dp[i][0]);
                        } else if (dp[i][0] - 1 == dp[j][0]) {
                            dp[i][1] += dp[j][1];
                        }
                    }
                }
            }

            int ans = 0;
            for (int i = 0; i < nums.length; i++) {
                if (longest == dp[i][0]) {
                    ans += dp[i][1];
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/minimize-malware-spread/description/
    static class Solution2 {
        public int minMalwareSpread(int[][] graph, int[] initial) {
            int n = graph.length;

            int[] colors = new int[n];
            Arrays.fill(colors, -1);

            var dfs1 = new Object() {
                void apply(int v, int c) {
                    colors[v] = c;
                    for (int u = 0; u < graph.length; u++) {
                        if (graph[v][u] == 1 && colors[u] == -1) {
                            apply(u, c);
                        }
                    }
                }
            };

            int C = 0;
            for (int v = 0; v < n; v++) {
                if (colors[v] == -1) {
                    dfs1.apply(v, C++);
                }
            }

            int[] size = new int[C];
            for (var color : colors) {
                size[color]++;
            }

            int[] colorCount = new int[C];
            for (int node : initial) {
                colorCount[colors[node]]++;
            }

            int ans = Integer.MAX_VALUE;
            for (int node : initial) {
                int c = colors[node];
                if (colorCount[c] == 1) {
                    if (ans == Integer.MAX_VALUE) {
                        ans = node;
                    } else if (size[c] > size[colors[ans]]) {
                        ans = node;
                    } else if (size[c] == size[colors[ans]] && node < ans) {
                        ans = node;
                    }
                }
            }
            if (ans == Integer.MAX_VALUE) {
                for (int node : initial) {
                    ans = Math.min(ans, node);
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/kth-ancestor-of-a-tree-node/description/
    class TreeAncestor {

        int maxD = 0;
        int[][] height;
        int[][] kthAncestor;

        public TreeAncestor(int n, int[] parent) {
            while ((1 << maxD) <= n) {
                maxD += 1;
            }

            kthAncestor = new int[n][maxD];
            for (int i = 0; i < n; i++) {
                Arrays.fill(kthAncestor[i], -1);
            }

            for (int i = 0; i < maxD; i++) {
                for (int j = 0; j < n; j++) {
                    if (i == 0) {
                        kthAncestor[j][i] = parent[j];
                    } else if (kthAncestor[j][i - 1] != -1) {
                        kthAncestor[j][i] = kthAncestor[kthAncestor[j][i - 1]][i - 1];
                    }
                }
            }
        }

        public int getKthAncestor(int node, int k) {
            for (int i = 0; i < maxD; i++) {
                if (((1 << i) & k) != 0) {
                    node = kthAncestor[node][i];
                    if (node == -1) {
                        break;
                    }
                }
            }
            return node;
        }
    }
}
