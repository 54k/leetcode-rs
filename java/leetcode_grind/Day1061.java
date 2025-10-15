package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day1061 {
    // https://leetcode.com/problems/coin-path/description/?envType=weekly-question&envId=2025-10-15
    static class Solution1 {
        public List<Integer> cheapestJump(int[] A, int B) {
            int[] next = new int[A.length];
            Arrays.fill(next, -1);
            long[] memo = new long[A.length];
            jump(A, B, 0, next, memo);
            List<Integer> res = new ArrayList<>();
            int i;
            for (i = 0; i < A.length && next[i] > 0; i = next[i]) {
                res.add(i + 1);
            }
            if (i == A.length - 1 && A[i] >= 0) {
                res.add(A.length);
            } else {
                return new ArrayList<>();
            }
            return res;
        }

        long jump(int[] A, int B, int i, int[] next, long[] memo) {
            if (memo[i] > 0) {
                return memo[i];
            }
            if (i == A.length - 1 && A[i] >= 0) {
                return A[i];
            }
            long min_cost = Integer.MAX_VALUE;
            for (int j = i + 1; j <= i + B && j < A.length; j++) {
                if (A[j] >= 0) {
                    long cost = A[i] + jump(A, B, j, next, memo);
                    if (cost < min_cost) {
                        min_cost = cost;
                        next[i] = j;
                    }
                }
            }
            memo[i] = min_cost;
            return min_cost;
        }
    }

    static class Solution2 {
        public List<Integer> cheapestJump(int[] A, int B) {
            int[] next = new int[A.length];
            long[] dp = new long[A.length];
            Arrays.fill(next, -1);
            List<Integer> res = new ArrayList<>();
            for (int i = A.length - 2; i >= 0; i--) {
                long min_cost = Integer.MAX_VALUE;
                for (int j = i + 1; j <= i + B && j < A.length; j++) {
                    if (A[j] >= 0) {
                        long cost = A[i] + dp[j];
                        if (cost < min_cost) {
                            min_cost = cost;
                            next[i] = j;
                        }
                    }
                }
                dp[i] = min_cost;
            }

            int i;
            for (i = 0; i < A.length && next[i] > 0; i = next[i]) {
                res.add(i + 1);
            }
            if (i == A.length - 1 && A[i] >= 0) {
                res.add(A.length);
            } else {
                return new ArrayList<>();
            }
            return res;
        }
    }

    // https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/description/?envType=daily-question&envId=2025-10-15
    static class Solution3 {
        public int maxIncreasingSubarrays(List<Integer> nums) {
            int n = nums.size();
            int cnt = 1;
            int precnt = 0;
            int ans = 0;

            for (int i = 1; i < n; i++) {
                if (nums.get(i) > nums.get(i - 1)) {
                    ++cnt;
                } else {
                    precnt = cnt;
                    cnt = 1;
                }
                ans = Math.max(ans, Math.min(precnt, cnt));
                ans = Math.max(ans, cnt / 2);
            }
            return ans;
        }
    }
}
