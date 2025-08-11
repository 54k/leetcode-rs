package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day996 {
    // https://leetcode.com/problems/range-product-queries-of-powers/description/?envType=daily-question&envId=2025-08-11
    static class Solution1 {
        static int MOD = 1000000007;

        public int[] productQueries(int n, int[][] queries) {
            List<Integer> bins = new ArrayList<>();
            int rep = 1;
            while (n > 0) {
                if (n % 2 == 1) {
                    bins.add(rep);
                }
                n /= 2;
                rep *= 2;
            }

            int[] ans = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                long cur = 1;
                int start = queries[i][0];
                int end = queries[i][1];
                for (int j = start; j <= end; j++) {
                    cur = (cur * bins.get(j)) % MOD;
                }
                ans[i] = (int) cur;
            }
            return ans;
        }
    }

    static class Solution2 {
        static int MOD = 1000000007;

        public int[] productQueries(int n, int[][] queries) {
            List<Integer> bins = new ArrayList<>();
            int rep = 1;
            while (n > 0) {
                if (n % 2 == 1) {
                    bins.add(rep);
                }
                n /= 2;
                rep *= 2;
            }

            int m = bins.size();
            int[][] results = new int[m][m];
            for (int i = 0; i < m; i++) {
                long cur = 1;
                for (int j = i; j < m; j++) {
                    cur = (cur * bins.get(j)) % MOD;
                    results[i][j] = (int) cur;
                }
            }

            int[] ans = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                ans[i] = results[queries[i][0]][queries[i][1]];
            }
            return ans;
        }
    }
}
