package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day373 {
    // https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/description
    static class Solution1 {
        public int garbageCollection1(String[] garbage, int[] travel) {
            int[] prefixSum = new int[travel.length + 1];
            prefixSum[1] = travel[0];
            for (int i = 1; i < travel.length; i++) {
                prefixSum[i + 1] = prefixSum[i] + travel[i];
            }

            Map<Character, Integer> garbageLastPos = new HashMap<>();
            Map<Character, Integer> garbageCount = new HashMap<>();

            for (int i = 0; i < garbage.length; i++) {
                for (char c : garbage[i].toCharArray()) {
                    garbageLastPos.put(c, i);
                    garbageCount.put(c, garbageCount.getOrDefault(c, 0) + 1);
                }
            }

            String garbageTypes = "MPG";
            int ans = 0;
            for (char c : garbageTypes.toCharArray()) {
                if (garbageCount.containsKey(c)) {
                    ans += prefixSum[garbageLastPos.get(c)] + garbageCount.get(c);
                }
            }
            return ans;
        }

        public int garbageCollection2(String[] garbage, int[] travel) {
            for (int i = 1; i < travel.length; i++) {
                travel[i] += travel[i - 1];
            }

            var ans = 0;
            var garbageLastPos = new HashMap<Character, Integer>();
            for (int i = 0; i < garbage.length; i++) {
                for (char c : garbage[i].toCharArray()) {
                    garbageLastPos.put(c, i);
                }
                ans += garbage[i].length();
            }

            for (char c : "MPG".toCharArray()) {
                ans += garbageLastPos.getOrDefault(c, 0) == 0 ? 0 : travel[garbageLastPos.get(c) - 1];
            }
            return ans;
        }

        public int garbageCollection3(String[] garbage, int[] travel) {
            var n = garbage.length;
            var ans = 0;

            for (var ch : new char[] { 'M', 'P', 'G' }) {
                var p = new int[n];
                var lastH = 0;
                for (int i = n - 1; i >= 0; i--) {
                    var h = garbage[i].replaceAll("[^" + ch + "]", "").length();
                    if (h > 0) {
                        lastH = Math.max(i, lastH);
                    }
                    p[i] = h + (i < n - 1 ? p[i + 1] : 0) + (i > 0 && lastH >= i ? travel[i - 1] : 0);
                }
                ans += p[0];
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/count-palindromic-subsequences/description/
    // https://leetcode.com/problems/count-palindromic-subsequences/solutions/2850466/c-java-python3-counting-prefixes-and-suffixes/
    class Solution2 {
        public int countPalindromes(String s) {
            int n = s.length();
            int[] cnt = new int[10];
            int[][][] prefix = new int[n][10][10];

            for (int i = 0; i < n; i++) {
                int c = s.charAt(i) - '0';
                if (i > 0) {
                    for (int j = 0; j < 10; j++) {
                        for (int k = 0; k < 10; k++) {
                            prefix[i][j][k] = prefix[i - 1][j][k];
                            if (c == k) {
                                prefix[i][j][k] += cnt[j];
                            }
                        }
                    }
                }
                cnt[c]++;
            }

            Arrays.fill(cnt, 0);

            int[][][] suffix = new int[n][10][10];
            for (int i = n - 1; i >= 0; i--) {
                int c = s.charAt(i) - '0';
                if (i < n - 1) {
                    for (int j = 0; j < 10; j++) {
                        for (int k = 0; k < 10; k++) {
                            suffix[i][j][k] = suffix[i + 1][j][k];
                            if (c == k) {
                                suffix[i][j][k] += cnt[j];
                            }
                        }
                    }
                }
                cnt[c]++;
            }

            int ans = 0;
            int MOD = 1_000_000_007;

            for (int i = 2; i < n - 2; i++) {
                for (int j = 0; j < 10; j++) {
                    for (int k = 0; k < 10; k++) {
                        ans = (int) ((ans + 1L * prefix[i - 1][j][k] * suffix[i + 1][j][k]) % MOD);
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/maximum-xor-product/description/
    // https://leetcode.com/problems/maximum-xor-product/solutions/4304377/no-multiplication/
    static class Solution3 {
        public int maximumXorProduct(long a, long b, int n) {
            int mod = 1_000_000_007;

            if (n > 0) {
                for (long bt = 1L << (n - 1); bt > 0; bt >>= 1) {
                    if ((Math.min(a, b) & bt) == 0) {
                        a ^= bt;
                        b ^= bt;
                    }
                }
            }

            return (int) (a % mod * (b % mod) % mod);
        }
    }

    // https://leetcode.com/problems/flood-fill/
    static class Solution4 {
        public int[][] floodFill(int[][] image, int sr, int sc, int color) {
            var orig = image[sr][sc];
            var dfs = new Object() {
                void dfs(int x, int y) {
                    if (image[x][y] == orig) {
                        image[x][y] = color;

                        if (x > 0) {
                            dfs(x - 1, y);
                        }
                        if (y > 0) {
                            dfs(x, y - 1);
                        }
                        if (x < image.length - 1) {
                            dfs(x + 1, y);
                        }
                        if (y < image[0].length - 1) {
                            dfs(x, y + 1);
                        }
                    }
                }
            };
            if (orig != color) {
                dfs.dfs(sr, sc);
            }
            return image;
        }
    }

    // https://leetcode.com/problems/diameter-of-binary-tree/
    static class TreeNode {
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

    static class Solution5 {
        public int diameterOfBinaryTree(TreeNode root) {
            var dfs = new Object() {
                int diameter;

                int apply(TreeNode node) {
                    if (node == null) {
                        return 0;
                    }
                    var left = apply(node.left);
                    var right = apply(node.right);
                    diameter = Math.max(diameter, left + right);
                    var max = Math.max(left, right);
                    return 1 + max;
                }
            };
            dfs.apply(root);
            return dfs.diameter;
        }
    }
}
