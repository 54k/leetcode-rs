package leetcode_grind;

import java.util.ArrayList;
import java.util.HashSet;

public class Day428 {
    // https://leetcode.com/problems/unique-binary-search-trees/description
    static class Solution1 {
        public int numTrees(int n) {
            int[] G = new int[n + 1];
            G[0] = 1;
            G[1] = 1;

            for (int i = 2; i <= n; i++) {
                for (int j = 1; j <= i; j++) {
                    G[i] += G[j - 1] * G[i - j];
                }
            }
            return G[n];
        }
    }

    // https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/
    static class Solution2 {
        public int areaOfMaxDiagonal(int[][] dimensions) {
            int ans = 0;
            double mx = 0.0;

            for (var d : dimensions) {
                double a = (double) d[0] * (double) d[0];
                double b = (double) d[1] * (double) d[1];
                double diag = Math.sqrt((double) a + (double) b);

                if (diag > mx) {
                    ans = d[0] * d[1];
                    mx = diag;
                } else if (diag == mx) {
                    ans = Math.max(ans, d[0] * d[1]);
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/intersection-of-two-arrays/
    static class Solution3 {
        public int[] intersection(int[] nums1, int[] nums2) {
            var s1 = new HashSet<Integer>();
            for (var n : nums1) {
                s1.add(n);
            }
            var res = new ArrayList<Integer>();
            for (var n : nums2) {
                if (s1.contains(n)) {
                    res.add(n);
                    s1.remove(n);
                }
            }
            return res.stream().mapToInt(Integer::valueOf).toArray();
        }
    }
}
