package leetcode_grind;

import java.util.Arrays;

public class Day448 {
    // https://leetcode.com/problems/partition-array-for-maximum-sum/description
    static class Solution1 {
        public int maxSumAfterPartitioning(int[] arr, int k) {
            int N = arr.length;
            int[] dp = new int[N];
            Arrays.fill(dp, -1);

            var rec = new Object() {
                int apply(int start) {
                    if (start >= N) {
                        return 0;
                    }
                    if (dp[start] > -1) {
                        return dp[start];
                    }

                    int currMax = 0, ans = 0;
                    int end = Math.min(N, start + k);
                    for (int i = start; i < end; i++) {
                        currMax = Math.max(currMax, arr[i]);
                        ans = Math.max(ans, currMax * (i - start + 1) + apply(i + 1));
                    }
                    return dp[start] = ans;
                }
            };

            return rec.apply(0);
        }
    }

}
