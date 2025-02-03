package leetcode_grind;

import java.util.HashMap;

public class Day806 {
    // https://leetcode.com/problems/distinct-numbers-in-each-subarray/description/
    static class Solution1 {
        public int[] distinctNumbers(int[] nums, int k) {
            var n = nums.length;
            var hm = new HashMap<Integer, Integer>();
            var ans = new int[n - k + 1];

            for (int i = 0; i < n; i++) {
                hm.put(nums[i], hm.getOrDefault(nums[i], 0) + 1);
                if (i >= k) {
                    hm.put(nums[i - k], hm.get(nums[i - k]) - 1);
                    if (hm.get(nums[i - k]) == 0) {
                        hm.remove(nums[i - k]);
                    }
                }

                if (i >= k - 1)
                    ans[i - k + 1] = hm.size();
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/sum-of-even-numbers-after-queries/description/
    static class Solution2 {
        public int[] sumEvenAfterQueries(int[] A, int[][] queries) {
            int S = 0;
            for (int x : A) {
                if (x % 2 == 0) {
                    S += x;
                }
            }

            int[] ans = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                int val = queries[i][0], index = queries[i][1];
                if (A[index] % 2 == 0)
                    S -= A[index];
                A[index] += val;
                if (A[index] % 2 == 0)
                    S += A[index];
                ans[i] = S;
            }

            return ans;
        }
    }
}
